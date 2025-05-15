use std::{thread, time::Duration};

use hidapi::{HidDevice, HidResult};

/// ID required as the first byte of all HID Reports.
pub const REPORT_ID: u8 = 0x03;

/// Number of bytes in each report.
pub const REPORT_LEN: usize = 16;

/// Number of bytes required for the header of each report.
const DEFAULT_HEADER_LEN: usize = 5;

/// Time to wait between each report of a message.
///
/// DPI operations seem to be particularly sensitive to this.
/// Assuming Core operates in a similar way, it seems like it uses a 150ms interval.
const REPORT_INTERVAL: Duration = Duration::from_millis(150);

pub const fn default_header(operation: u8, index: u8) -> [u8; DEFAULT_HEADER_LEN] {
    [REPORT_ID, operation, 0xFB, index, 0x01]
}

#[derive(Clone, PartialEq, Eq)]
struct Report {
    data: Vec<u8>,
}

impl Report {
    fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// Sends the feature report to the given mouse.
    ///
    /// Sleeps for [`REPORT_INTERVAL`] after sending to allow time to process requests.
    fn send(&self, mouse: &HidDevice) -> HidResult<()> {
        mouse.send_feature_report(self.data.as_slice())?;
        thread::sleep(REPORT_INTERVAL);
        Ok(())
    }
}

impl std::fmt::Debug for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Report")
            .field("data", &format_args!("{:02X?}", self.data))
            .finish()
    }
}

/// A message consists of one or more reports which may be sent to a mouse to perform some
/// operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    reports: Vec<Report>,
}

impl Message {
    fn new(reports: Vec<Report>) -> Self {
        Self { reports }
    }

    /// Sends each report in the current message.
    ///
    /// Sleeps for [`REPORT_INTERVAL`] after sending each report to allow time to process.
    pub fn send(self, mouse: &HidDevice) -> HidResult<()> {
        for report in self.reports {
            report.send(mouse)?;
        }

        Ok(())
    }
}

pub struct MessageBuilder<'header> {
    num_reports: u8,
    blocks: Vec<Vec<u8>>,
    header_fn: Box<dyn FnMut(u8) -> Vec<u8> + 'header>,
}

impl<'header> MessageBuilder<'header> {
    pub fn new(operation_id: u8, num_reports: u8) -> Self {
        Self {
            num_reports,
            blocks: Vec::new(),
            header_fn: Box::new(move |i| default_header(operation_id, i).to_vec()),
        }
    }

    pub fn with_header(mut self, header_fn: impl FnMut(u8) -> Vec<u8> + 'header) -> Self {
        self.header_fn = Box::new(header_fn);
        self
    }

    pub fn push(mut self, byte: u8) -> Self {
        self.blocks.push(vec![byte]);
        self
    }

    pub fn push_block(mut self, block: &[u8]) -> Self {
        self.blocks.push(block.to_vec());
        self
    }

    pub fn build(mut self) -> MessageBuilderResult<Message> {
        if self.num_reports == 0 {
            return Err(MessageBuilderError::DataLenError { block_i: 0 })
        }

        let mut reports = Vec::new();
        let mut report = (self.header_fn)(0);
        if report.len() > REPORT_LEN {
            return Err(MessageBuilderError::HeaderLenError { report_i: 0, header_len: report.len() });
        }

        let mut i = 1;
        for (block_i, block) in self.blocks.into_iter().enumerate() {
            if report.len() + block.len() > REPORT_LEN {
                if i == self.num_reports {
                    return Err(MessageBuilderError::DataLenError { block_i });
                }

                // Not enough room for current block
                report.resize(REPORT_LEN, 0x00);
                reports.push(Report::new(report));

                report = (self.header_fn)(i);
                if report.len() > REPORT_LEN {
                    return Err(MessageBuilderError::HeaderLenError {
                        report_i: i,
                        header_len: report.len(),
                    });
                }

                i += 1;
                continue;
            }

            if report.len() + block.len() > REPORT_LEN {
                // Current block too long
                return Err(MessageBuilderError::BlockLenError {
                    block_i,
                    block_len: block.len(),
                });
            }

            report.extend(block); // Push block to current report
        }

        // Push remaining data
        report.resize(REPORT_LEN, 0x00);
        reports.push(Report::new(report));

        // Generate remaining blank reports
        while i < self.num_reports {
            report = (self.header_fn)(i);
            if report.len() > REPORT_LEN {
                return Err(MessageBuilderError::HeaderLenError {
                    report_i: i,
                    header_len: report.len(),
                });
            }
            i += 1;
            report.resize(REPORT_LEN, 0x00);
            reports.push(Report::new(report));
        }

        Ok(Message::new(reports))
    }
}

type MessageBuilderResult<T> = Result<T, MessageBuilderError>;

#[derive(Debug)]
pub enum MessageBuilderError {
    DataLenError { block_i: usize },
    HeaderLenError { report_i: u8, header_len: usize },
    BlockLenError { block_i: usize, block_len: usize },
}

impl std::fmt::Display for MessageBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageBuilderError::DataLenError { block_i } => {
                write!(f, "Message full, not enough room to write block {block_i}")
            }
            MessageBuilderError::HeaderLenError { report_i: i, header_len } => {
                write!(f, "Header of report {i} too long ({header_len})")
            }
            MessageBuilderError::BlockLenError { block_i, block_len } => {
                write!(f, "Block {block_i} too long ({block_len})")
            }
        }
    }
}

impl std::error::Error for MessageBuilderError {}

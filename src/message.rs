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

pub struct MessageBuilder<'a> {
    operation: u8,
    num_reports: u8,
    data: Vec<Vec<u8>>,
    i: usize,
    header_len: usize,
    header_fn: Option<Box<dyn FnMut(u8) -> Vec<u8> + 'a>>,
}

#[cfg_attr(debug_assertions, allow(dead_code))]
impl<'a> MessageBuilder<'a> {
    pub fn new(operation: u8, num_reports: u8) -> Self {
        Self {
            operation,
            num_reports,
            data: vec![Vec::new(); num_reports as usize],
            i: 0,
            header_len: DEFAULT_HEADER_LEN,
            header_fn: None,
        }
    }

    /// The header is the first few bytes of every report in a message.
    /// Usually these bytes are the same, but sometimes a custom header is required.
    ///
    /// If a custom header is required, the supplied `header_fn` closure MUST always return a vector
    /// with length `header_len` to avoid unexpected behaviour.
    pub fn new_with_header(
        operation: u8,
        num_reports: u8,
        header_len: usize,
        header_fn: impl FnMut(u8) -> Vec<u8> + 'a,
    ) -> Self {
        Self {
            operation,
            num_reports,
            data: vec![Vec::new(); num_reports as usize],
            i: 0,
            header_len,
            header_fn: Some(Box::new(header_fn)),
        }
    }

    fn data_len(&self) -> usize {
        REPORT_LEN - self.header_len
    }

    pub fn capacity(&self) -> usize {
        self.num_reports as usize * self.data_len()
    }

    pub fn len(&self) -> usize {
        self.i * self.data_len() + self.data[self.i].len()
    }

    /// Moves to the next internal report
    pub fn incr_report(mut self) -> ReportBuilderResult<Self> {
        if self.i + 1 == self.num_reports as usize {
            return Err(ReportBuilderError::LengthError);
        }

        self.i += 1;
        Ok(self)
    }

    /// Pushes a single byte of data to a report
    pub fn push(mut self, byte: u8) -> ReportBuilderResult<Self> {
        if self.data[self.i].len() == self.data_len() {
            self = self.incr_report()?;
        }

        self.data[self.i].push(byte);
        Ok(self)
    }

    pub fn push_block(mut self, block: &[u8]) -> ReportBuilderResult<Self> {
        if block.len() > self.data_len() {
            return Err(ReportBuilderError::LengthError);
        }

        let remaining_reports = self.num_reports as usize - self.i - 1;
        let remaining_in_curr = self.data_len() - self.data[self.i].len();
        if remaining_reports == 0 && remaining_in_curr < block.len() {
            return Err(ReportBuilderError::LengthError);
        }

        if remaining_in_curr < block.len() {
            self = self.incr_report().unwrap(); // We have checked that remaining_reports > 0
        }

        self.data[self.i].extend(block);
        Ok(self)
    }

    pub fn build(mut self) -> Message {
        let mut reports = Vec::with_capacity(self.num_reports as usize);

        for (i, bytes) in self.data.into_iter().enumerate() {
            let mut report = Report {
                data: match self.header_fn {
                    Some(ref mut header_fn) => header_fn(i as u8),
                    None => default_header(self.operation, i as u8).to_vec(),
                },
            };

            report.data.extend(bytes);
            assert!(
                report.data.len() <= REPORT_LEN,
                "Report longer than maximum length!"
            );
            report.data.resize(REPORT_LEN, 0x0);
            reports.push(report);
        }

        Message::new(reports)
    }
}

type ReportBuilderResult<T> = Result<T, ReportBuilderError>;

#[derive(Debug, Clone, Copy)]
pub enum ReportBuilderError {
    LengthError,
}

impl std::fmt::Display for ReportBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReportBuilderError::LengthError => write!(
                f,
                "Pushing to this report would exceed its maximum capacity",
            ),
        }
    }
}

impl std::error::Error for ReportBuilderError {}

use hidapi::{HidDevice, HidResult};

/// ID required as the first byte of all HID Reports
pub const REPORT_ID: u8 = 0x03;

/// Number of bytes in each report
pub const REPORT_LEN: usize = 16;

/// Number of bytes required for the header of each report
const DEFAULT_HEADER_LEN: usize = 5;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    // TODO: Make non-public
    pub data: Vec<u8>,
}

impl Report {
    pub const fn default_header(operation: u8, index: u8) -> [u8; DEFAULT_HEADER_LEN] {
        [REPORT_ID, operation, 0xFB, index, 0x01]
    }

    pub fn send(&self, mouse: &HidDevice) -> HidResult<()> {
        mouse.send_feature_report(self.data.as_slice())
    }
}

pub struct ReportBuilder<'a> {
    operation: u8,
    num_reports: u8,
    data: Vec<Vec<u8>>,
    i: usize,
    header_len: usize,
    header_fn: Option<Box<dyn FnMut(u8) -> Vec<u8> + 'a>>,
}

impl<'a> ReportBuilder<'a> {
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

    pub fn extend(mut self, bytes: &[u8]) -> ReportBuilderResult<Self> {
        // Exit without pushing anything if we don't have room to push everything
        if self.capacity() - self.len() < bytes.len() {
            return Err(ReportBuilderError::LengthError);
        }

        for byte in bytes {
            self = self.push(*byte)?;
        }

        Ok(self)
    }

    pub fn extend_contiguous(mut self, bytes: &[u8]) -> ReportBuilderResult<Self> {
        if bytes.len() > self.data_len() {
            return Err(ReportBuilderError::LengthError);
        }

        let remaining_reports = self.num_reports as usize - self.i - 1;
        let remaining_in_curr = self.data_len() - self.data[self.i].len();
        if remaining_reports == 0 && remaining_in_curr < bytes.len() {
            return Err(ReportBuilderError::LengthError);
        }

        if remaining_in_curr < bytes.len() {
            self = self.incr_report().unwrap(); // We have checked that remaining_reports > 0
        }

        self.data[self.i].extend(bytes);
        Ok(self)
    }

    pub fn build(mut self) -> Vec<Report> {
        let mut reports = Vec::with_capacity(self.num_reports as usize);

        for (i, bytes) in self.data.into_iter().enumerate() {
            let mut report = Report {
                data: match self.header_fn {
                    Some(ref mut header_fn) => header_fn(i as u8),
                    None => Report::default_header(self.operation, i as u8).to_vec(),
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

        reports
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

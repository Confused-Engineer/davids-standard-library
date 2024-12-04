#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

#[cfg(target_os="windows")]
use std::os::windows::fs::MetadataExt;

use crate::log::LogFile;

impl LogFile
{

    /// Set the directory to the same directory as the binary, keeps the name "output.log"  
    /// # Examples
    /// ```
    /// let mut test = davids_standard_library::log::LogFile::new();
    /// test.default_file_path();
    /// test.write_log("success");
    /// ```

    pub fn default_file_path(&mut self) -> std::io::Result<&mut Self> {
        
        self.dir = crate::env::get_exe_dir()?;
        self.update_path();
        Ok(self)
    }

    /// Set the directory for the log file to a custom one, retains the name "output.log"
    /// # Examples
    /// ```
    /// let mut test = davids_standard_library::log::LogFile::new();
    /// test.custom_file_path("..\\");
    /// test.write_log("success");
    /// ```

    pub fn custom_file_path(&mut self, filepath: String) -> &mut Self {
        self.dir = filepath;
        self.update_path();
        self
    }

    /// Changes the name of the log file to "log.log"
    /// # Examples
    /// ```
    /// let mut test = davids_standard_library::log::LogFile::new();
    /// test.file_name("log.log");
    /// test.write_log("success");
    /// ```

    pub fn file_name(&mut self, filename: String) -> &mut Self {
        self.name = filename;
        self.update_path();
        self
    }

    /// Writes out the custom message to the log file
    /// # Examples
    /// ```
    /// let mut test = davids_standard_library::log::LogFile::new();
    /// test.write_log("success");
    /// ```

    pub fn write_log(&mut self, message: &str) -> std::io::Result<()>
    {
        use std::io::Write;
        if let Ok(mut file) = std::fs::OpenOptions::new().write(true).create(true).append(true).open(&self.path)
        {
            let new_line = format!("{} \n", message);
            println!("{}", new_line);
            file.write(new_line.as_bytes())?;
            file.flush()?;

        }
        
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Could Not Open/Create File to write to"))
 
    }

    /// Removes the log file, using the write log again will bring it back. 
    /// # Examples
    /// ```
    /// let mut test = davids_standard_library::log::LogFile::new();
    /// test.write_log("success");
    /// test.clear_log();
    /// ```
    pub fn clear_log(&mut self) -> std::io::Result<()>
    {
        std::fs::remove_file(&self.path)?;
        Ok(())
    }


    /// Removes the log file if larger than size in bytes, using the write log again will bring it back. 
    /// # Examples
    /// ```
    /// let mut test = davids_standard_library::log::LogFile::new();
    /// test.write_log("success");
    /// test.clear_log_larger_than_size(1000);
    /// ```
    /// This would only delete the log file if it is larger than 1kb
    #[cfg(target_os="windows")]
    pub fn clear_log_larger_than_size(&mut self, size: u64) -> std::io::Result<()>
    {
        let filesize = std::fs::metadata(&self.path)?.file_size();
        if filesize > size
        {
            self.clear_log()?;
        }
        Ok(())
    }
    

    //Updates the information needed so that writing out the logs is up to date
    fn update_path(&mut self) -> &mut Self
    {
        //println!("{}", format!("{}{}", self.dir, self.name));
        self.path = std::ffi::OsString::from(format!("{}{}", self.dir, self.name));
        self
    }
}
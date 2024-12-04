#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

/// # Return the current users Home Directory
/// Windows: C:\Users\USERNAME
/// Linux/Mac: /home/USERNAME
pub fn get_home() -> std::io::Result<String>
{
    #[cfg(target_os="windows")]
    let home = std::env::var_os("userprofile");
    #[cfg(not(target_os="windows"))]
    let home = std::env::var_os("HOME");
    
    if let Some(home) = home
    {
        if let Ok(home) = home.into_string()
        {
            return Ok(home)
        }   
    }

    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Path is Empty"))
}

/// Returns the directory of the executable, excluding the executable itself 
pub fn get_exe_dir() -> std::io::Result<String>
{
    let exe_path = std::env::current_exe()?.display().to_string();
    
    #[cfg(target_os="windows")]
    let exe = exe_path.split("\\").last();

    #[cfg(not(target_os="windows"))]
    let exe = exe_path.split("/").last();

    if let Some(exe) = exe
    {
        return Ok(std::env::current_exe()?.display().to_string().replace(exe, ""))
    }

    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Path is Empty"))
}


/// sets the working directory (pwd) to the directory of the executable
pub fn set_exe_dir() -> std::io::Result<()>
{
    let exe_path = std::env::current_exe()?.display().to_string();
    
    #[cfg(target_os="windows")]
    let exe = exe_path.split("\\").last();

    #[cfg(not(target_os="windows"))]
    let exe = exe_path.split("/").last();

    if let Some(exe) = exe
    {
        let current_path = std::env::current_exe()?.display().to_string().replace(exe, "");
        return Ok(std::env::set_current_dir(current_path)?);
    }

    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Path is Empty"))
}
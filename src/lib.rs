extern crate hdf5_sys;
use hdf5_sys::*;
use std::path::Path;
use std::ffi::CString;

const H5P_DEFAULT: hid_t = 0;
const H5F_ACC_RDWR: u32 = 1;
const H5F_ACC_CREAT: u32 = 2;

/*
macro_rules! Str {
	() => ()
}
*/
pub struct PropertyList {

}
pub fn file(filename: &Path, _plist: &PropertyList) -> hid_t {
	
	
	let filename_cstr: CString = CString::new(filename.to_str().expect("bad string")).expect("bad cstring");
	
	if filename.exists() {
		print!("found file: {:?}. opening...", filename);
        unsafe { H5Fopen(filename_cstr.into_raw(), H5F_ACC_RDWR, H5P_DEFAULT) }
	}
	else {
		print!("file not found. creating {:?}...", filename);
        unsafe { H5Fcreate(filename_cstr.into_raw(), H5F_ACC_CREAT, H5P_DEFAULT, H5P_DEFAULT) }
	}

}

pub fn new_file(filename: &Path, _plist: &PropertyList) -> hid_t {
	
	
	let filename_cstr: CString = CString::new(filename.to_str().expect("bad string")).expect("bad cstring");
	
    unsafe { H5Fcreate(filename_cstr.into_raw(), 0, H5P_DEFAULT, H5P_DEFAULT) }
    
}

pub fn close_file(id: hid_t) -> herr_t {
	unsafe { H5Fclose(id) }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;
    

    #[test]
    fn version() {
        let (mut majnum, mut minnum, mut relnum) = (0, 0, 0);
        assert!(unsafe { H5get_libversion(&mut majnum, &mut minnum, &mut relnum) } >= 0);
        assert_eq!((majnum, minnum, relnum), (1, 10, 4));
    }
    #[test]
    fn test_no_file() {
    	let fname = Path::new("nofile.h5");
    	let result: hid_t = file(fname, & PropertyList{});
    	assert!(result >= 0, "error: {}", result);
    	assert!(fname.exists(), "could not find {}", fname.display());
    	assert_eq!(close_file(result), 0, "could not close file");
    	remove_file(fname).expect("couldn't remove file");
    }
    #[test]
    fn test_new_file() {
    	let fname = Path::new("newfile.h5");
    	let result: hid_t = new_file(fname, & PropertyList{});
    	assert!(result >= 0, "error: {}", result);
    	assert!(fname.exists(), "could not find {}", fname.display());
    	assert_eq!(close_file(result), 0, "could not close file");
    	remove_file(fname).expect("couldn't remove file");
    }
}

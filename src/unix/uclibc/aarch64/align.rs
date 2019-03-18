s! {
    // FIXME this is actually a union
    #[cfg_attr(target_pointer_width = "32",
               repr(align(4)))]
    #[cfg_attr(target_pointer_width = "64",
               repr(align(8)))]
    pub struct sem_t {
        #[cfg(target_pointer_width = "32")]
        __size: [::c_char; 16],
        #[cfg(target_pointer_width = "64")]
        __size: [::c_char; 32],
    }

    // long alignment
    #[cfg_attr(target_pointer_width = "32",
               repr(align(4)))]
    #[cfg_attr(target_pointer_width = "64",
               repr(align(8)))]
    pub struct pthread_rwlockattr_t {
        __size: [::c_char; 8],
    }

    // long alignment
    #[cfg_attr(target_pointer_width = "32",
               repr(align(4)))]
    #[cfg_attr(target_pointer_width = "64",
               repr(align(8)))]
    pub struct siginfo_t {
        pub si_signo: ::c_int,
        pub si_errno: ::c_int,
        pub si_code: ::c_int,
        pub _pad: [::c_int; 29],
    }

    // long long alignment
    #[repr(align(8))]
    pub struct statfs {
        pub f_type: ::c_uint,
        pub f_bsize: ::c_uint,
        pub f_blocks: ::c_uint,
        pub __pad1: ::c_uint,
        pub f_bfree: ::c_uint,
        pub __pad2: ::c_uint,
        pub f_bavail: ::c_uint,
        pub __pad3: ::c_uint,
        pub f_files: ::c_uint,
        pub __pad4: ::c_uint,
        pub f_ffree: ::c_uint,
        pub __pad5: ::c_uint,
        pub f_fsid: ::fsid_t,
        pub f_namelen: ::c_uint,
        pub f_frsize: ::c_uint,
        pub f_flags: ::c_uint,
        pub f_spare: [::c_uint; 4],
    }
}

s_no_extra_traits! {
    // long long alignment
    #[allow(missing_debug_implementations)]
    #[repr(align(8))]
    pub struct dirent {
        pub d_ino: ::c_uint,
        pub __pad1: ::c_uint,
        pub d_off: ::c_int,
        pub __pad2: ::c_int,
        pub d_reclen: ::c_ushort,
        pub d_type: ::c_uchar,
        pub d_name: [::c_char; 256],
    }
}

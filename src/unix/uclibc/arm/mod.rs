pub type c_char = u8;
pub type wchar_t = ::c_int;
pub type c_long = i32;
pub type c_ulong = u32;
pub type time_t = ::c_long;

pub type clock_t = ::c_long;
pub type fsblkcnt_t = ::c_ulong;
pub type fsfilcnt_t = ::c_ulong;
pub type ino_t = ::c_ulong;
pub type off_t = ::c_long;
pub type pthread_t = ::c_ulong;
pub type rlim_t = ::c_ulong;
// shmid_ds test12;
pub type suseconds_t = ::c_long;
// termios test17;

pub type nlink_t = ::c_uint;
pub type blksize_t = ::c_long;
pub type blkcnt_t = ::c_long;

pub const O_CLOEXEC: c_int = 0o2000000;
pub const RLIM_INFINITY: rlim_t = !0;
pub const __SIZEOF_PTHREAD_ATTR_T: usize = 36;
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 24;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_COND_T: usize = 48;
pub const __SIZEOF_PTHREAD_COND_COMPAT_T: usize = 12;
pub const __SIZEOF_PTHREAD_CONDATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_RWLOCK_T: usize = 32;
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: usize = 8;
pub const __SIZEOF_PTHREAD_BARRIER_T: usize = 20;
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: usize = 4;

s! {
    pub struct msghdr {
        msg_name: *mut ::c_void,
        msg_namelen: ::socklen_t,
        msg_iov: *mut ::iovec,
        msg_iovlen: ::c_int,
        msg_control: *mut ::c_void,
        msg_controllen: ::socklen_t,
        msg_flags: ::c_int,
    }

    pub struct pthread_attr_t {
        __size: [::c_long; 9],
    }

    pub struct stat {
        pub st_dev: ::c_ulonglong,
        __pad1: ::c_ushort,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::c_ulonglong,
        __pad2: ::c_ushort,
        pub st_size: ::off_t,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt_t,
        pub st_atim: ::timespec,
        pub st_mtim: ::timespec,
        pub st_ctim: ::timespec,
        __unused4: ::c_ulong,
        __unused5: ::c_ulong,
    }

    pub struct sigset_t {
        __val: [::c_ulong; 2],
    }

    pub struct sigaction {
        pub sa_handler: ::sighandler_t,
        pub sa_flags: ::c_ulong,
        pub sa_restorer: *mut ::c_void;
        pub sa_mask: sigset_t,
    }

    pub struct sem_t {
        __size: [::c_long; 4],
    }

    pub struct termios {
        pub c_iflag: ::tcflag_t,
        pub c_oflag: ::tcflag_t,
        pub c_cflag: ::tcflag_t,
        pub c_lflag: ::tcflag_t,
        pub c_line: ::cc_t,
        pub c_cc: [::cc_t; ::NCCS],
        pub c_ispeed: ::speed_t,
        pub c_ospeed: ::speed_t,
    }

    pub struct siginfo_t {
        pub si_signo: ::c_int,
        pub si_code: ::c_int,
        pub si_errno: ::c_int,
        pub _pad: [::c_int; 29],
    }

    pub struct stack_t {
        pub ss_sp: *mut ::c_void,
        ss_flags: ::c_int,
        ss_size: ::size_t,
    }

// struct msqid_ds
// {
//   struct ipc_perm msg_perm;
//   __time_t msg_stime;
//   unsigned long int __unused1;
//   __time_t msg_rtime;
//   unsigned long int __unused2;
//   __time_t msg_ctime;
//   unsigned long int __unused3;
//   unsigned long int __msg_cbytes;
//   msgqnum_t msg_qnum;
//   msglen_t msg_qbytes;
//   __pid_t msg_lspid;
//   __pid_t msg_lrpid;
//   unsigned long int __unused4;
//   unsigned long int __unused5;
// };    
}

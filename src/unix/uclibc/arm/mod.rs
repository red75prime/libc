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
// pthread_t test9;
// rlim_t test10;
// sem_t test11;
// shmid_ds test12;
// siginfo_t test13;
// sigset_t test14;
// stack_t test15;
// suseconds_t test16;
// termios test17;

s! {
    pub struct msghdr {
        msg_name: *mut ::c_void,
        msg_namelen: ::socklen_t,
        msg_iov: *mut ::iovec,
        msg_iovlen: ::c_int,
        msg_control: *mut ::c_void,
        msg_controllen: ::socklen_t,
        msg_flags: :c_int,
    }

    pub struct pthread_attr_t {
        __size: [::c_long; 9],
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

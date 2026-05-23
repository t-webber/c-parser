



extern void __assert_fail (const char *__assertion, const char *__file,
      unsigned int __line, const char *__function)
     ;


extern void __assert_perror_fail (int __errnum, const char *__file,
      unsigned int __line, const char *__function)
     ;




extern void __assert (const char *__assertion, const char *__file, int __line)
     ;






extern _Bool __assert_single_arg (_Bool);



 


typedef unsigned char __u_char;
typedef unsigned short int __u_short;
typedef unsigned int __u_int;
typedef unsigned long int __u_long;


typedef signed char __int8_t;
typedef unsigned char __uint8_t;
typedef signed short int __int16_t;
typedef unsigned short int __uint16_t;
typedef signed int __int32_t;
typedef unsigned int __uint32_t;

typedef signed long int __int64_t;
typedef unsigned long int __uint64_t;






typedef __int8_t __int_least8_t;
typedef __uint8_t __uint_least8_t;
typedef __int16_t __int_least16_t;
typedef __uint16_t __uint_least16_t;
typedef __int32_t __int_least32_t;
typedef __uint32_t __uint_least32_t;
typedef __int64_t __int_least64_t;
typedef __uint64_t __uint_least64_t;



typedef long int __quad_t;
typedef unsigned long int __u_quad_t;







typedef long int __intmax_t;
typedef unsigned long int __uintmax_t;


typedef unsigned long int __dev_t;
typedef unsigned int __uid_t;
typedef unsigned int __gid_t;
typedef unsigned long int __ino_t;
typedef unsigned long int __ino64_t;
typedef unsigned int __mode_t;
typedef unsigned long int __nlink_t;
typedef long int __off_t;
typedef long int __off64_t;
typedef int __pid_t;
typedef struct { int __val[2]; } __fsid_t;
typedef long int __clock_t;
typedef unsigned long int __rlim_t;
typedef unsigned long int __rlim64_t;
typedef unsigned int __id_t;
typedef long int __time_t;
typedef unsigned int __useconds_t;
typedef long int __suseconds_t;
typedef long int __suseconds64_t;

typedef int __daddr_t;
typedef int __key_t;


typedef int __clockid_t;


typedef void * __timer_t;


typedef long int __blksize_t;




typedef long int __blkcnt_t;
typedef long int __blkcnt64_t;


typedef unsigned long int __fsblkcnt_t;
typedef unsigned long int __fsblkcnt64_t;


typedef unsigned long int __fsfilcnt_t;
typedef unsigned long int __fsfilcnt64_t;


typedef long int __fsword_t;

typedef long int __ssize_t;


typedef long int __syscall_slong_t;

typedef unsigned long int __syscall_ulong_t;



typedef __off64_t __loff_t;
typedef char *__caddr_t;


typedef long int __intptr_t;


typedef unsigned int __socklen_t;




typedef int __sig_atomic_t;
typedef __int8_t int8_t;
typedef __int16_t int16_t;
typedef __int32_t int32_t;
typedef __int64_t int64_t;


typedef __uint8_t uint8_t;
typedef __uint16_t uint16_t;
typedef __uint32_t uint32_t;
typedef __uint64_t uint64_t;



typedef __int_least8_t int_least8_t;
typedef __int_least16_t int_least16_t;
typedef __int_least32_t int_least32_t;
typedef __int_least64_t int_least64_t;


typedef __uint_least8_t uint_least8_t;
typedef __uint_least16_t uint_least16_t;
typedef __uint_least32_t uint_least32_t;
typedef __uint_least64_t uint_least64_t;





typedef signed char int_fast8_t;

typedef long int int_fast16_t;
typedef long int int_fast32_t;
typedef long int int_fast64_t;
typedef unsigned char uint_fast8_t;

typedef unsigned long int uint_fast16_t;
typedef unsigned long int uint_fast32_t;
typedef unsigned long int uint_fast64_t;
typedef long int intptr_t;


typedef unsigned long int uintptr_t;
typedef __intmax_t intmax_t;
typedef __uintmax_t uintmax_t;









typedef long unsigned int size_t;


typedef __builtin_va_list __gnuc_va_list;






typedef struct
{
  int __count;
  union
  {
    unsigned int __wch;
    char __wchb[4];
  } __value;
} __mbstate_t;




typedef struct _G_fpos_t
{
  __off_t __pos;
  __mbstate_t __state;
} __fpos_t;
typedef struct _G_fpos64_t
{
  __off64_t __pos;
  __mbstate_t __state;
} __fpos64_t;



struct _IO_FILE;
typedef struct _IO_FILE __FILE;



struct _IO_FILE;


typedef struct _IO_FILE FILE;

struct _IO_FILE;
struct _IO_marker;
struct _IO_codecvt;
struct _IO_wide_data;




typedef void _IO_lock_t;





struct _IO_FILE
{
  int _flags;


  char *_IO_read_ptr;
  char *_IO_read_end;
  char *_IO_read_base;
  char *_IO_write_base;
  char *_IO_write_ptr;
  char *_IO_write_end;
  char *_IO_buf_base;
  char *_IO_buf_end;


  char *_IO_save_base;
  char *_IO_backup_base;
  char *_IO_save_end;

  struct _IO_marker *_markers;

  struct _IO_FILE *_chain;

  int _fileno;
  int _flags2:24;

  char _short_backupbuf[1];
  __off_t _old_offset;


  unsigned short _cur_column;
  signed char _vtable_offset;
  char _shortbuf[1];

  _IO_lock_t *_lock;







  __off64_t _offset;

  struct _IO_codecvt *_codecvt;
  struct _IO_wide_data *_wide_data;
  struct _IO_FILE *_freeres_list;
  void *_freeres_buf;
  struct _IO_FILE **_prevchain;
  int _mode;

  int _unused3;

  __uint64_t _total_written;




  char _unused2[12 * sizeof (int) - 5 * sizeof (void *)];
};


typedef __ssize_t cookie_read_function_t (void *__cookie, char *__buf,
                                          size_t __nbytes);







typedef __ssize_t cookie_write_function_t (void *__cookie, const char *__buf,
                                           size_t __nbytes);







typedef int cookie_seek_function_t (void *__cookie, __off64_t *__pos, int __w);


typedef int cookie_close_function_t (void *__cookie);






typedef struct _IO_cookie_io_functions_t
{
  cookie_read_function_t *read;
  cookie_write_function_t *write;
  cookie_seek_function_t *seek;
  cookie_close_function_t *close;
} cookie_io_functions_t;





typedef __gnuc_va_list va_list;
typedef __off_t off_t;
typedef __ssize_t ssize_t;






typedef __fpos_t fpos_t;
extern FILE *stdin;
extern FILE *stdout;
extern FILE *stderr;






extern int remove (const char *__filename) ;

extern int rename (const char *__old, const char *__new) ;



extern int renameat (int __oldfd, const char *__old, int __newfd,
       const char *__new) ;
extern int fclose (FILE *__stream) ;
extern FILE *tmpfile (void)
   ;
extern char *tmpnam (char[20])  ;




extern char *tmpnam_r (char __s[20])  ;
extern char *tempnam (const char *__dir, const char *__pfx)
   ;






extern int fflush (FILE *__stream);
extern int fflush_unlocked (FILE *__stream);
extern FILE *fopen (const char *__restrict __filename,
      const char *__restrict __modes)
   ;




extern FILE *freopen (const char *__restrict __filename,
        const char *__restrict __modes,
        FILE *__restrict __stream) ;
extern FILE *fdopen (int __fd, const char *__modes) 
   ;





extern FILE *fopencookie (void *__restrict __magic_cookie,
     const char *__restrict __modes,
     cookie_io_functions_t __io_funcs) 
   ;




extern FILE *fmemopen (void *__s, size_t __len, const char *__modes)
   ;




extern FILE *open_memstream (char **__bufloc, size_t *__sizeloc) 
   ;
extern void setbuf (FILE *__restrict __stream, char *__restrict __buf) 
  ;



extern int setvbuf (FILE *__restrict __stream, char *__restrict __buf,
      int __modes, size_t __n) ;




extern void setbuffer (FILE *__restrict __stream, char *__restrict __buf,
         size_t __size) ;


extern void setlinebuf (FILE *__stream) ;







extern int fprintf (FILE *__restrict __stream,
      const char *__restrict __format, ...) ;




extern int printf (const char *__restrict __format, ...);

extern int sprintf (char *__restrict __s,
      const char *__restrict __format, ...) ;





extern int vfprintf (FILE *__restrict __s, const char *__restrict __format,
       __gnuc_va_list __arg) ;




extern int vprintf (const char *__restrict __format, __gnuc_va_list __arg);

extern int vsprintf (char *__restrict __s, const char *__restrict __format,
       __gnuc_va_list __arg) ;



extern int snprintf (char *__restrict __s, size_t __maxlen,
       const char *__restrict __format, ...)
     ;

extern int vsnprintf (char *__restrict __s, size_t __maxlen,
        const char *__restrict __format, __gnuc_va_list __arg)
     ;





extern int vasprintf (char **__restrict __ptr, const char *__restrict __f,
        __gnuc_va_list __arg)
      ;
extern int __asprintf (char **__restrict __ptr,
         const char *__restrict __fmt, ...)
      ;
extern int asprintf (char **__restrict __ptr,
       const char *__restrict __fmt, ...)
      ;




extern int vdprintf (int __fd, const char *__restrict __fmt,
       __gnuc_va_list __arg)
     ;
extern int dprintf (int __fd, const char *__restrict __fmt, ...)
     ;







extern int fscanf (FILE *__restrict __stream,
     const char *__restrict __format, ...) ;




extern int scanf (const char *__restrict __format, ...) ;

extern int sscanf (const char *__restrict __s,
     const char *__restrict __format, ...) ;









extern int fscanf (FILE *__restrict __stream, const char *__restrict __format, ...) __asm__ ("" "__isoc23_fscanf")

                                ;
extern int scanf (const char *__restrict __format, ...) __asm__ ("" "__isoc23_scanf")
                              ;
extern int sscanf (const char *__restrict __s, const char *__restrict __format, ...) __asm__ ("" "__isoc23_sscanf") 

                      ;
extern int vfscanf (FILE *__restrict __s, const char *__restrict __format,
      __gnuc_va_list __arg)
     ;





extern int vscanf (const char *__restrict __format, __gnuc_va_list __arg)
      ;


extern int vsscanf (const char *__restrict __s,
      const char *__restrict __format, __gnuc_va_list __arg)
     ;






extern int vfscanf (FILE *__restrict __s, const char *__restrict __format, __gnuc_va_list __arg) __asm__ ("" "__isoc23_vfscanf")



     ;
extern int vscanf (const char *__restrict __format, __gnuc_va_list __arg) __asm__ ("" "__isoc23_vscanf")

      ;
extern int vsscanf (const char *__restrict __s, const char *__restrict __format, __gnuc_va_list __arg) __asm__ ("" "__isoc23_vsscanf") 



     ;
extern int fgetc (FILE *__stream) ;
extern int getc (FILE *__stream) ;





extern int getchar (void);






extern int getc_unlocked (FILE *__stream) ;
extern int getchar_unlocked (void);
extern int fgetc_unlocked (FILE *__stream) ;







extern int fputc (int __c, FILE *__stream) ;
extern int putc (int __c, FILE *__stream) ;





extern int putchar (int __c);
extern int fputc_unlocked (int __c, FILE *__stream) ;







extern int putc_unlocked (int __c, FILE *__stream) ;
extern int putchar_unlocked (int __c);






extern int getw (FILE *__stream) ;


extern int putw (int __w, FILE *__stream) ;







extern char *fgets (char *__restrict __s, int __n, FILE *__restrict __stream)
     ;
extern __ssize_t __getdelim (char **__restrict __lineptr,
                             size_t *__restrict __n, int __delimiter,
                             FILE *__restrict __stream) ;
extern __ssize_t getdelim (char **__restrict __lineptr,
                           size_t *__restrict __n, int __delimiter,
                           FILE *__restrict __stream) ;


extern __ssize_t getline (char **__restrict __lineptr,
                          size_t *__restrict __n,
                          FILE *__restrict __stream) ;







extern int fputs (const char *__restrict __s, FILE *__restrict __stream)
  ;





extern int puts (const char *__s);






extern int ungetc (int __c, FILE *__stream) ;






extern size_t fread (void *__restrict __ptr, size_t __size,
       size_t __n, FILE *__restrict __stream)
  ;




extern size_t fwrite (const void *__restrict __ptr, size_t __size,
        size_t __n, FILE *__restrict __s) ;
extern size_t fread_unlocked (void *__restrict __ptr, size_t __size,
         size_t __n, FILE *__restrict __stream)
  ;
extern size_t fwrite_unlocked (const void *__restrict __ptr, size_t __size,
          size_t __n, FILE *__restrict __stream)
  ;







extern int fseek (FILE *__stream, long int __off, int __whence)
  ;




extern long int ftell (FILE *__stream) ;




extern void rewind (FILE *__stream) ;
extern int fseeko (FILE *__stream, __off_t __off, int __whence)
  ;




extern __off_t ftello (FILE *__stream) ;
extern int fgetpos (FILE *__restrict __stream, fpos_t *__restrict __pos)
  ;




extern int fsetpos (FILE *__stream, const fpos_t *__pos) ;
extern void clearerr (FILE *__stream) ;

extern int feof (FILE *__stream) ;

extern int ferror (FILE *__stream) ;



extern void clearerr_unlocked (FILE *__stream) ;
extern int feof_unlocked (FILE *__stream) ;
extern int ferror_unlocked (FILE *__stream) ;







extern void perror (const char *__s) ;




extern int fileno (FILE *__stream) ;




extern int fileno_unlocked (FILE *__stream) ;
extern int pclose (FILE *__stream) ;





extern FILE *popen (const char *__command, const char *__modes)
   ;






extern char *ctermid (char *__s) 
  ;
extern void flockfile (FILE *__stream) ;



extern int ftrylockfile (FILE *__stream) ;


extern void funlockfile (FILE *__stream) ;
extern int __uflow (FILE *);
extern int __overflow (FILE *, int);






typedef int wchar_t;


typedef struct
  {
    int quot;
    int rem;
  } div_t;



typedef struct
  {
    long int quot;
    long int rem;
  } ldiv_t;





__extension__ typedef struct
  {
    long long int quot;
    long long int rem;
  } lldiv_t;
extern size_t __ctype_get_mb_cur_max (void)  ;



extern double atof (const char *__nptr)
      ;

extern int atoi (const char *__nptr)
      ;

extern long int atol (const char *__nptr)
      ;



__extension__ extern long long int atoll (const char *__nptr)
      ;



extern double strtod (const char *__restrict __nptr,
        char **__restrict __endptr)
     ;



extern float strtof (const char *__restrict __nptr,
       char **__restrict __endptr) ;

extern long double strtold (const char *__restrict __nptr,
       char **__restrict __endptr)
     ;
extern long int strtol (const char *__restrict __nptr,
   char **__restrict __endptr, int __base)
     ;

extern unsigned long int strtoul (const char *__restrict __nptr,
      char **__restrict __endptr, int __base)
     ;



__extension__
extern long long int strtoq (const char *__restrict __nptr,
        char **__restrict __endptr, int __base)
     ;

__extension__
extern unsigned long long int strtouq (const char *__restrict __nptr,
           char **__restrict __endptr, int __base)
     ;




__extension__
extern long long int strtoll (const char *__restrict __nptr,
         char **__restrict __endptr, int __base)
     ;

__extension__
extern unsigned long long int strtoull (const char *__restrict __nptr,
     char **__restrict __endptr, int __base)
     ;






extern long int strtol (const char *__restrict __nptr, char **__restrict __endptr, int __base) __asm__ ("" "__isoc23_strtol") 


     ;
extern unsigned long int strtoul (const char *__restrict __nptr, char **__restrict __endptr, int __base) __asm__ ("" "__isoc23_strtoul") 



     ;

__extension__
extern long long int strtoq (const char *__restrict __nptr, char **__restrict __endptr, int __base) __asm__ ("" "__isoc23_strtoll") 


     ;
__extension__
extern unsigned long long int strtouq (const char *__restrict __nptr, char **__restrict __endptr, int __base) __asm__ ("" "__isoc23_strtoull") 



     ;

__extension__
extern long long int strtoll (const char *__restrict __nptr, char **__restrict __endptr, int __base) __asm__ ("" "__isoc23_strtoll") 


     ;
__extension__
extern unsigned long long int strtoull (const char *__restrict __nptr, char **__restrict __endptr, int __base) __asm__ ("" "__isoc23_strtoull") 



     ;
extern int strfromd (char *__dest, size_t __size, const char *__format,
       double __f)
     ;

extern int strfromf (char *__dest, size_t __size, const char *__format,
       float __f)
     ;

extern int strfroml (char *__dest, size_t __size, const char *__format,
       long double __f)
     ;
extern char *l64a (long int __n)  ;


extern long int a64l (const char *__s)
      ;










typedef __u_char u_char;
typedef __u_short u_short;
typedef __u_int u_int;
typedef __u_long u_long;
typedef __quad_t quad_t;
typedef __u_quad_t u_quad_t;
typedef __fsid_t fsid_t;


typedef __loff_t loff_t;




typedef __ino_t ino_t;
typedef __dev_t dev_t;




typedef __gid_t gid_t;




typedef __mode_t mode_t;




typedef __nlink_t nlink_t;




typedef __uid_t uid_t;
typedef __pid_t pid_t;





typedef __id_t id_t;
typedef __daddr_t daddr_t;
typedef __caddr_t caddr_t;





typedef __key_t key_t;










typedef __clock_t clock_t;







typedef __clockid_t clockid_t;
typedef __time_t time_t;






typedef __timer_t timer_t;



typedef unsigned long int ulong;
typedef unsigned short int ushort;
typedef unsigned int uint;







typedef __uint8_t u_int8_t;
typedef __uint16_t u_int16_t;
typedef __uint32_t u_int32_t;
typedef __uint64_t u_int64_t;


typedef int register_t ;
static __inline __uint16_t
__bswap_16 (__uint16_t __bsx)
{

  return __builtin_bswap16 (__bsx);



}






static __inline __uint32_t
__bswap_32 (__uint32_t __bsx)
{

  return __builtin_bswap32 (__bsx);



}
__extension__ static __inline __uint64_t
__bswap_64 (__uint64_t __bsx)
{

  return __builtin_bswap64 (__bsx);



}
static __inline __uint16_t
__uint16_identity (__uint16_t __x)
{
  return __x;
}

static __inline __uint32_t
__uint32_identity (__uint32_t __x)
{
  return __x;
}

static __inline __uint64_t
__uint64_identity (__uint64_t __x)
{
  return __x;
}











typedef struct
{
  unsigned long int __val[(1024 / (8 * sizeof (unsigned long int)))];
} __sigset_t;


typedef __sigset_t sigset_t;










struct timeval
{




  __time_t tv_sec;
  __suseconds_t tv_usec;

};

struct timespec
{



  __time_t tv_sec;




  __syscall_slong_t tv_nsec;
};



typedef __suseconds_t suseconds_t;





typedef long int __fd_mask;
typedef struct
  {






    __fd_mask __fds_bits[1024 / (8 * (int) sizeof (__fd_mask))];


  } fd_set;






typedef __fd_mask fd_mask;

extern int select (int __nfds, fd_set *__restrict __readfds,
     fd_set *__restrict __writefds,
     fd_set *__restrict __exceptfds,
     struct timeval *__restrict __timeout);
extern int pselect (int __nfds, fd_set *__restrict __readfds,
      fd_set *__restrict __writefds,
      fd_set *__restrict __exceptfds,
      const struct timespec *__restrict __timeout,
      const __sigset_t *__restrict __sigmask);






typedef __blksize_t blksize_t;






typedef __blkcnt_t blkcnt_t;



typedef __fsblkcnt_t fsblkcnt_t;



typedef __fsfilcnt_t fsfilcnt_t;

typedef union
{
  __extension__ unsigned long long int __value64;
  struct
  {
    unsigned int __low;
    unsigned int __high;
  } __value32;
} __atomic_wide_counter;




typedef struct __pthread_internal_list
{
  struct __pthread_internal_list *__prev;
  struct __pthread_internal_list *__next;
} __pthread_list_t;

typedef struct __pthread_internal_slist
{
  struct __pthread_internal_slist *__next;
} __pthread_slist_t;
struct __pthread_mutex_s
{
  int __lock;
  unsigned int __count;
  int __owner;

  unsigned int __nusers;



  int __kind;

  short __spins;
  short __unused;
  __pthread_list_t __list;
};
struct __pthread_rwlock_arch_t
{
  unsigned int __readers;
  unsigned int __writers;
  unsigned int __wrphase_futex;
  unsigned int __writers_futex;
  unsigned int __pad3;
  unsigned int __pad4;

  int __cur_writer;
  int __shared;
  unsigned long int __pad1;
  unsigned long int __pad2;


  unsigned int __flags;
};




struct __pthread_cond_s
{
  __atomic_wide_counter __wseq;
  __atomic_wide_counter __g1_start;
  unsigned int __g_size[2] ;
  unsigned int __g1_orig_size;
  unsigned int __wrefs;
  unsigned int __g_signals[2];
  unsigned int __unused_initialized_1;
  unsigned int __unused_initialized_2;
};

typedef unsigned int __tss_t;
typedef unsigned long int __thrd_t;

typedef struct
{
  int __data ;
} __once_flag;



typedef unsigned long int pthread_t;




typedef union
{
  char __size[4];
  int __align;
} pthread_mutexattr_t;




typedef union
{
  char __size[4];
  int __align;
} pthread_condattr_t;



typedef unsigned int pthread_key_t;



typedef int pthread_once_t;


union pthread_attr_t
{
  char __size[56];
  long int __align;
};

typedef union pthread_attr_t pthread_attr_t;




typedef union
{
  struct __pthread_mutex_s __data;
  char __size[40];
  long int __align;
} pthread_mutex_t;


typedef union
{
  struct __pthread_cond_s __data;
  char __size[48];
  __extension__ long long int __align;
} pthread_cond_t;





typedef union
{
  struct __pthread_rwlock_arch_t __data;
  char __size[56];
  long int __align;
} pthread_rwlock_t;

typedef union
{
  char __size[8];
  long int __align;
} pthread_rwlockattr_t;





typedef volatile int pthread_spinlock_t;




typedef union
{
  char __size[32];
  long int __align;
} pthread_barrier_t;

typedef union
{
  char __size[4];
  int __align;
} pthread_barrierattr_t;









extern long int random (void) ;


extern void srandom (unsigned int __seed) ;





extern char *initstate (unsigned int __seed, char *__statebuf,
   size_t __statelen) ;



extern char *setstate (char *__statebuf) ;







struct random_data
  {
    int32_t *fptr;
    int32_t *rptr;
    int32_t *state;
    int rand_type;
    int rand_deg;
    int rand_sep;
    int32_t *end_ptr;
  };

extern int random_r (struct random_data *__restrict __buf,
       int32_t *__restrict __result) ;

extern int srandom_r (unsigned int __seed, struct random_data *__buf)
     ;

extern int initstate_r (unsigned int __seed, char *__restrict __statebuf,
   size_t __statelen,
   struct random_data *__restrict __buf)
     ;

extern int setstate_r (char *__restrict __statebuf,
         struct random_data *__restrict __buf)
     ;





extern int rand (void) ;

extern void srand (unsigned int __seed) ;



extern int rand_r (unsigned int *__seed) ;







extern double drand48 (void) ;
extern double erand48 (unsigned short int __xsubi[3]) ;


extern long int lrand48 (void) ;
extern long int nrand48 (unsigned short int __xsubi[3])
     ;


extern long int mrand48 (void) ;
extern long int jrand48 (unsigned short int __xsubi[3])
     ;


extern void srand48 (long int __seedval) ;
extern unsigned short int *seed48 (unsigned short int __seed16v[3])
     ;
extern void lcong48 (unsigned short int __param[7]) ;





struct drand48_data
  {
    unsigned short int __x[3];
    unsigned short int __old_x[3];
    unsigned short int __c;
    unsigned short int __init;
    __extension__ unsigned long long int __a;

  };


extern int drand48_r (struct drand48_data *__restrict __buffer,
        double *__restrict __result) ;
extern int erand48_r (unsigned short int __xsubi[3],
        struct drand48_data *__restrict __buffer,
        double *__restrict __result) ;


extern int lrand48_r (struct drand48_data *__restrict __buffer,
        long int *__restrict __result)
     ;
extern int nrand48_r (unsigned short int __xsubi[3],
        struct drand48_data *__restrict __buffer,
        long int *__restrict __result)
     ;


extern int mrand48_r (struct drand48_data *__restrict __buffer,
        long int *__restrict __result)
     ;
extern int jrand48_r (unsigned short int __xsubi[3],
        struct drand48_data *__restrict __buffer,
        long int *__restrict __result)
     ;


extern int srand48_r (long int __seedval, struct drand48_data *__buffer)
     ;

extern int seed48_r (unsigned short int __seed16v[3],
       struct drand48_data *__buffer) ;

extern int lcong48_r (unsigned short int __param[7],
        struct drand48_data *__buffer)
     ;


extern __uint32_t arc4random (void)
      ;


extern void arc4random_buf (void *__buf, size_t __size)
     ;



extern __uint32_t arc4random_uniform (__uint32_t __upper_bound)
      ;




extern void *malloc (size_t __size) 
      ;

extern void *calloc (size_t __nmemb, size_t __size)
      ;






extern void *realloc (void *__ptr, size_t __size)
     ;


extern void free (void *__ptr) ;
extern void free_sized (void *__ptr, size_t __size) ;




extern void free_aligned_sized (void *__ptr, size_t __alignment, size_t __size)
     ;
extern void *reallocarray (void *__ptr, size_t __nmemb, size_t __size)
     
     
    ;


extern void *reallocarray (void *__ptr, size_t __nmemb, size_t __size)
     ;










extern void *alloca (size_t __size) ;











extern void *valloc (size_t __size) 
      ;




extern int posix_memalign (void **__memptr, size_t __alignment, size_t __size)
      ;




extern void *aligned_alloc (size_t __alignment, size_t __size)
     
      ;



extern void abort (void) ;



extern int atexit (void (*__func) (void)) ;







extern int at_quick_exit (void (*__func) (void)) ;






extern int on_exit (void (*__func) (int __status, void *__arg), void *__arg)
     ;





extern void exit (int __status) ;





extern void quick_exit (int __status) ;





extern void _Exit (int __status) ;




extern char *getenv (const char *__name)  ;
extern int putenv (char *__string) ;





extern int setenv (const char *__name, const char *__value, int __replace)
     ;


extern int unsetenv (const char *__name) ;






extern int clearenv (void) ;
extern char *mktemp (char *__template) ;
extern int mkstemp (char *__template)  ;
extern int mkstemps (char *__template, int __suffixlen)  ;
extern char *mkdtemp (char *__template)  ;
extern int system (const char *__command) ;
extern char *realpath (const char *__restrict __name,
         char *__restrict __resolved)  ;






typedef int (*__compar_fn_t) (const void *, const void *);
extern void *bsearch (const void *__key, const void *__base,
        size_t __nmemb, size_t __size, __compar_fn_t __compar)
      ;
extern void qsort (void *__base, size_t __nmemb, size_t __size,
     __compar_fn_t __compar) ;
extern int abs (int __x)  ;
extern long int labs (long int __x)  ;


__extension__ extern long long int llabs (long long int __x)
      ;
extern div_t div (int __numer, int __denom)
      ;
extern ldiv_t ldiv (long int __numer, long int __denom)
      ;


__extension__ extern lldiv_t lldiv (long long int __numer,
        long long int __denom)
      ;
extern char *ecvt (double __value, int __ndigit, int *__restrict __decpt,
     int *__restrict __sign)  ;




extern char *fcvt (double __value, int __ndigit, int *__restrict __decpt,
     int *__restrict __sign)  ;




extern char *gcvt (double __value, int __ndigit, char *__buf)
      ;




extern char *qecvt (long double __value, int __ndigit,
      int *__restrict __decpt, int *__restrict __sign)
      ;
extern char *qfcvt (long double __value, int __ndigit,
      int *__restrict __decpt, int *__restrict __sign)
      ;
extern char *qgcvt (long double __value, int __ndigit, char *__buf)
      ;




extern int ecvt_r (double __value, int __ndigit, int *__restrict __decpt,
     int *__restrict __sign, char *__restrict __buf,
     size_t __len) ;
extern int fcvt_r (double __value, int __ndigit, int *__restrict __decpt,
     int *__restrict __sign, char *__restrict __buf,
     size_t __len) ;

extern int qecvt_r (long double __value, int __ndigit,
      int *__restrict __decpt, int *__restrict __sign,
      char *__restrict __buf, size_t __len)
     ;
extern int qfcvt_r (long double __value, int __ndigit,
      int *__restrict __decpt, int *__restrict __sign,
      char *__restrict __buf, size_t __len)
     ;





extern int mblen (const char *__s, size_t __n) ;


extern int mbtowc (wchar_t *__restrict __pwc,
     const char *__restrict __s, size_t __n) ;


extern int wctomb (char *__s, wchar_t __wchar) ;



extern size_t mbstowcs (wchar_t *__restrict __pwcs,
   const char *__restrict __s, size_t __n) 
    ;

extern size_t wcstombs (char *__restrict __s,
   const wchar_t *__restrict __pwcs, size_t __n)
     
  
  ;






extern int rpmatch (const char *__response)  ;
extern int getsubopt (char **__restrict __optionp,
        char *const *__restrict __tokens,
        char **__restrict __valuep)
      ;
extern int getloadavg (double __loadavg[], int __nelem)
     ;
typedef __once_flag once_flag;



extern void call_once (once_flag *__flag, void (*__func)(void));



extern size_t memalignment (const void *__p);





















struct tm
{
  int tm_sec;
  int tm_min;
  int tm_hour;
  int tm_mday;
  int tm_mon;
  int tm_year;
  int tm_wday;
  int tm_yday;
  int tm_isdst;


  long int tm_gmtoff;
  const char *tm_zone;




};







struct itimerspec
  {
    struct timespec it_interval;
    struct timespec it_value;
  };
struct sigevent;
struct __locale_struct
{

  struct __locale_data *__locales[13];


  const unsigned short int *__ctype_b;
  const int *__ctype_tolower;
  const int *__ctype_toupper;


  const char *__names[13];
};

typedef struct __locale_struct *__locale_t;

typedef __locale_t locale_t;




extern clock_t clock (void) ;



extern time_t time (time_t *__timer) ;


extern double difftime (time_t __time1, time_t __time0);


extern time_t mktime (struct tm *__tp) ;
extern size_t strftime (char *__restrict __s, size_t __maxsize,
   const char *__restrict __format,
   const struct tm *__restrict __tp)
   ;
extern size_t strftime_l (char *__restrict __s, size_t __maxsize,
     const char *__restrict __format,
     const struct tm *__restrict __tp,
     locale_t __loc) ;
extern struct tm *gmtime (const time_t *__timer) ;



extern struct tm *localtime (const time_t *__timer) ;
extern struct tm *gmtime_r (const time_t *__restrict __timer,
       struct tm *__restrict __tp) ;



extern struct tm *localtime_r (const time_t *__restrict __timer,
          struct tm *__restrict __tp) ;
extern char *asctime (const struct tm *__tp) ;



extern char *ctime (const time_t *__timer) ;
extern char *asctime_r (const struct tm *__restrict __tp,
   char *__restrict __buf) ;



extern char *ctime_r (const time_t *__restrict __timer,
        char *__restrict __buf) ;
extern char *__tzname[2];
extern int __daylight;
extern long int __timezone;




extern char *tzname[2];



extern void tzset (void) ;



extern int daylight;
extern long int timezone;
extern time_t timegm (struct tm *__tp) ;
extern time_t timelocal (struct tm *__tp) ;







extern int dysize (int __year) ;
extern int nanosleep (const struct timespec *__requested_time,
        struct timespec *__remaining);


extern int clock_getres (clockid_t __clock_id, struct timespec *__res) ;


extern int clock_gettime (clockid_t __clock_id, struct timespec *__tp)
     ;


extern int clock_settime (clockid_t __clock_id, const struct timespec *__tp)
     ;
extern int clock_nanosleep (clockid_t __clock_id, int __flags,
       const struct timespec *__req,
       struct timespec *__rem);
extern int clock_getcpuclockid (pid_t __pid, clockid_t *__clock_id) ;




extern int timer_create (clockid_t __clock_id,
    struct sigevent *__restrict __evp,
    timer_t *__restrict __timerid) ;


extern int timer_delete (timer_t __timerid) ;



extern int timer_settime (timer_t __timerid, int __flags,
     const struct itimerspec *__restrict __value,
     struct itimerspec *__restrict __ovalue) ;


extern int timer_gettime (timer_t __timerid, struct itimerspec *__value)
     ;
extern int timer_getoverrun (timer_t __timerid) ;






extern int timespec_get (struct timespec *__ts, int __base)
     ;
extern int timespec_getres (struct timespec *__ts, int __base)
     ;





static unsigned naive_simd(unsigned word) {
        unsigned count = 0;
        for (unsigned i = 0; i < 32; ++i) count += word >> i & 1;
        return count;
}

static unsigned naive(unsigned word) {
        unsigned count = 0;
        while (word) {
                count += word & 1;
                word >>= 1;
        }
        return count;
}

static unsigned kerninghan(unsigned word) {
        unsigned count = 0;
        while (word) {
                word &= word - 1;
                count += 1;
        }
        return count;
}

static unsigned popcount(unsigned word) {
        return (unsigned)__builtin_popcount(word);
}

static unsigned t[1024] = {0};

static unsigned lookup(unsigned word) {
        return t[word & 0xff] + t[word >> 8 & 0xff] + t[word >> 16 & 0xff]
               + t[word >> 24 & 0xff];
}

static unsigned linpcnt(unsigned word) {
        long x = word;
        x = x - ((x >> 1) & 0x55555555);
        x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
        x = (x + (x >> 4)) & 0x0F0F0F0F;
        x = x + (x >> 8);
        x = x + (x >> 16);
        return x & 0x3F;
}

static unsigned lg(unsigned x) {
        if (x > 1) return (32 - (unsigned)__builtin_clz(x - 1));
        return 0;
}

static unsigned
vtpc_global(unsigned word, const unsigned mask[], const unsigned mask_size) {
        unsigned m = 0;
        while (word > 0) {
                m += mask[word % mask_size];
                word /= mask_size;
        };
        return m;
}






static unsigned vtpc1(const unsigned word) { return vtpc_global(word, t, 1 << 1); } static unsigned vtpc2(const unsigned word) { return vtpc_global(word, t, 1 << 2); } static unsigned vtpc3(const unsigned word) { return vtpc_global(word, t, 1 << 3); } static unsigned vtpc4(const unsigned word) { return vtpc_global(word, t, 1 << 4); } static unsigned vtpc5(const unsigned word) { return vtpc_global(word, t, 1 << 5); } static unsigned vtpc6(const unsigned word) { return vtpc_global(word, t, 1 << 6); } static unsigned vtpc7(const unsigned word) { return vtpc_global(word, t, 1 << 7); } static unsigned vtpc8(const unsigned word) { return vtpc_global(word, t, 1 << 8); } static unsigned vtpc9(const unsigned word) { return vtpc_global(word, t, 1 << 9); } static unsigned vtpc10(const unsigned word) { return vtpc_global(word, t, 1 << 10); }

    static unsigned divconq(unsigned word) {
        word = (word & 0x55555555) + ((word >> 1) & 0x55555555);
        word = (word & 0x33333333) + ((word >> 2) & 0x33333333);
        word = (word + (word >> 4)) & 0x0F0F0F0F;
        word = (word * 0x01010101) >> 24;
        return word;
}

static unsigned long rfc(unsigned n) {
        unsigned long counter = 0;

        for (int bit = 0; bit < 32; bit++) {
                unsigned long coin = 1LL << (bit + 1);
                unsigned long prev = 1LL << bit;

                unsigned long complet_blocks = (n + 1) / coin;
                counter += complet_blocks * prev;

                unsigned long remainder = (n + 1) % coin;
                if (remainder > prev) { counter += (remainder - prev); }
        }
        return counter;
}

static unsigned rf(unsigned word) {
        if (word == 0) return 0;
        return (unsigned)(rfc(word) - rfc(word - 1));
}



static unsigned long long int time_ns(void) {
        struct timespec start;
        clock_gettime(
                     1
                                    , &start);
        return start.tv_sec * 1e9 + start.tv_nsec;
}

typedef unsigned (*func)(unsigned);

const func FNS[18] = {linpcnt,
                          lookup,
                          popcount,
                          kerninghan,
                          naive,
                          vtpc1,
                          vtpc2,
                          vtpc3,
                          vtpc4,
                          vtpc5,
                          vtpc6,
                          vtpc7,
                          vtpc8,
                          vtpc9,
                          vtpc10,
                          divconq,
                          rf,
                          naive_simd};
const char *const FN_NAMES[18] = {"linpcnt",
                                      "lookup",
                                      "popcount",
                                      "kerninghan",
                                      "naive",
                                      "vtpc1",
                                      "vtpc2",
                                      "vtpc3",
                                      "vtpc4",
                                      "vtpc5",
                                      "vtpc6",
                                      "vtpc7",
                                      "vtpc8",
                                      "vtpc9",
                                      "vtpc10",
                                      "divconq",
                                      "rfc",
                                      "naive_simd"};

static void init(void) {
        for (int bit = 0; bit < 1024; ++bit) {
                t[bit] = (bit & 1) + t[bit >> 1];
        }
}

int main(int argc, char **argv) {
        
       ((void) sizeof (__assert_single_arg (
       argc == 3
       )), __extension__ ({ if (
       argc == 3
       ) ; else __assert_fail (
       "argc == 3"
       , "/home/b/.files/.work/hamming/src/popcount.c", 159, __extension__ __PRETTY_FUNCTION__); }))
                        ;

        init();

        const size_t iter = (size_t)atoll(argv[1]);
        const size_t exp_len = (size_t)atoll(argv[2]);

        unsigned int *const words = malloc(sizeof(unsigned int) * iter);
        unsigned int *const value = malloc(sizeof(unsigned int) * iter);

        unsigned long long times[18] = {0};
        unsigned long long max[18] = {0};
        unsigned long long min[18] = {0};

        for (size_t i = 0; i < exp_len; ++i) {
                for (size_t j = 0; j < iter; ++j) {

                        srand((unsigned)time(
                                            ((void *)0)
                                                ) + (unsigned)j);
                        words[j] = (unsigned)rand();
                }
                for (size_t fid = 0; fid < 18; ++fid) {
                        unsigned long long start = time_ns();
                        for (size_t j = 0; j < iter; ++j) {
                                value[j] = FNS[fid](words[j]);
                        }
                        unsigned long long time = (time_ns() - start) / 1000;
                        times[fid] += time;
                        if (!min[fid] || min[fid] > time) min[fid] = time;
                        if (!max[fid] || max[fid] < time) max[fid] = time;
                        unsigned int expected;
                        for (size_t j = 0; j < iter; ++j) {
                                expected = popcount(words[j]);
                                if (value[j] == popcount(words[j])) continue;
                                printf("%s failed on word %d: returned %d, "
                                       "expected %d\n",
                                       FN_NAMES[fid],
                                       words[j],
                                       value[j],
                                       expected);
                        }
                }
        }

        for (int fid = 0; fid < 18; ++fid) {
                unsigned long long avg = times[fid] / exp_len;
                printf(
                    "%-10s: \x1b[%dm%-5llu\x1b[0m (min: \x1b[%dm%-5llu\x1b[0m, "
                    "max: \x1b[%dm%-5llu\x1b[0m) ms\n",
                    FN_NAMES[fid],
                    avg < 1000 ? 32 : 31,
                    avg,
                    min[fid] < 1000 ? 32 : 31,
                    min[fid],
                    max[fid] < 1000 ? 32 : 31,
                    max[fid]);
        }

        return 0;
}

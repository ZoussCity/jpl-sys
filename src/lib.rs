pub const JPL_EPHEM_START_JD: u32 = 0;
pub const JPL_EPHEM_END_JD: u32 = 8;
pub const JPL_EPHEM_STEP: u32 = 16;
pub const JPL_EPHEM_N_CONSTANTS: u32 = 24;
pub const JPL_EPHEM_AU_IN_KM: u32 = 28;
pub const JPL_EPHEM_EARTH_MOON_RATIO: u32 = 36;
pub const JPL_EPHEM_IPT_ARRAY: u32 = 44;
pub const JPL_EPHEM_EPHEMERIS_VERSION: u32 = 224;
pub const JPL_EPHEM_KERNEL_SIZE: u32 = 228;
pub const JPL_EPHEM_KERNEL_RECORD_SIZE: u32 = 232;
pub const JPL_EPHEM_KERNEL_NCOEFF: u32 = 236;
pub const JPL_EPHEM_KERNEL_SWAP_BYTES: u32 = 240;
pub const JPL_EPH_OUTSIDE_RANGE: i32 = -1;
pub const JPL_EPH_READ_ERROR: i32 = -2;
pub const JPL_EPH_QUANTITY_NOT_IN_EPHEMERIS: i32 = -3;
pub const JPL_EPH_INVALID_INDEX: i32 = -5;
pub const JPL_EPH_FSEEK_ERROR: i32 = -6;
pub const JPL_INIT_NO_ERROR: u32 = 0;
pub const JPL_INIT_FILE_NOT_FOUND: i32 = -1;
pub const JPL_INIT_FSEEK_FAILED: i32 = -2;
pub const JPL_INIT_FREAD_FAILED: i32 = -3;
pub const JPL_INIT_FREAD2_FAILED: i32 = -4;
pub const JPL_INIT_FREAD5_FAILED: i32 = -10;
pub const JPL_INIT_FILE_CORRUPT: i32 = -5;
pub const JPL_INIT_MEMORY_FAILURE: i32 = -6;
pub const JPL_INIT_FREAD3_FAILED: i32 = -7;
pub const JPL_INIT_FREAD4_FAILED: i32 = -8;
pub const JPL_INIT_NOT_CALLED: i32 = -9;

extern "C" {
    /// ```text
    /// this function does the initial prep work for use of binary JPL
    /// ephemerides.
    ///   const char *ephemeris_filename = full path/filename of the binary
    ///       ephemeris (on the Willmann-Bell CDs,  this is UNIX.200, 405,
    ///       or 406)
    ///   char nam[][6] = array of constant names (max 6 characters each)
    ///       You can pass nam=NULL if you don't care about the names
    ///   double *val = array of values of constants
    ///       You can pass val=NULL if you don't care about the constants
    ///   Return value is a pointer to the jpl_eph_data structure
    ///   NULL is returned if the file isn't opened or memory isn't alloced
    ///   Errors can be determined with the above jpl_init_error_code( )
    /// ```
    pub fn jpl_init_ephemeris(
        ephemeris_filename: *const ::std::os::raw::c_char,
        nam: *mut [::std::os::raw::c_char; 6usize],
        val: *mut f64,
    ) -> *mut ::std::os::raw::c_void;

    /// ```text
    /// this function closes files and frees up memory allocated by the
    /// jpl_init_ephemeris( ) function.
    /// ```
    pub fn jpl_close_ephemeris(ephem: *mut ::std::os::raw::c_void);

    /// ```text
    /// This subroutine reads and interpolates the jpl planetary ephemeris file
    ///
    ///    Calling sequence parameters:
    ///
    ///    Input:
    ///
    ///        et2[] double, 2-element JED epoch at which interpolation
    ///              is wanted.  Any combination of et2[0]+et2[1] which falls
    ///              within the time span on the file is a permissible epoch.
    ///
    ///               a. for ease in programming, the user may put the
    ///                  entire epoch in et2[0] and set et2[1]=0.0
    ///
    ///               b. for maximum interpolation accuracy, set et2[0] =
    ///                  the most recent midnight at or before interpolation
    ///                  epoch and set et2[1] = fractional part of a day
    ///                  elapsed between et2[0] and epoch.
    ///
    ///               c. as an alternative, it may prove convenient to set
    ///                  et2[0] = some fixed epoch, such as start of integration,
    ///                  and et2[1] = elapsed interval between then and epoch.
    ///
    ///       list   13-element integer array specifying what interpolation
    ///              is wanted for each of the "bodies" on the file.
    ///
    ///                        list[i]=0, no interpolation for body i
    ///                               =1, position only
    ///                               =2, position and velocity
    ///
    ///              the designation of the astronomical bodies by i is:
    ///
    ///                        i = 0: mercury
    ///                          = 1: venus
    ///                          = 2: earth-moon barycenter
    ///                          = 3: mars
    ///                          = 4: jupiter
    ///                          = 5: saturn
    ///                          = 6: uranus
    ///                          = 7: neptune
    ///                          = 8: pluto
    ///                          = 9: geocentric moon
    ///                          =10: nutations in lon & obliq (if on file)
    ///                          =11: lunar librations (if on file)
    ///                          =12: lunar mantle omegas
    ///                          =13: TT-TDB (if on file)
    ///
    /// Note that I've not actually seen case 12 yet.  It probably doesn't work.
    ///
    ///    output:
    ///
    ///    pv[][6]   double array that will contain requested interpolated
    ///              quantities.  The body specified by list[i] will have its
    ///              state in the array starting at pv[i][0]  (on any given
    ///              call, only those words in 'pv' which are affected by the
    ///              first 10 'list' entries (and by list(11) if librations are
    ///              on the file) are set.  The rest of the 'pv' array
    ///              is untouched.)  The order of components in pv[][] is:
    ///              pv[][0]=x,....pv[][5]=dz.
    ///
    ///              All output vectors are referenced to the earth mean
    ///              equator and equinox of epoch. The moon state is always
    ///              geocentric; the other nine states are either heliocentric
    ///              or solar-system barycentric, depending on the setting of
    ///              global variables (see below).
    ///
    ///              Lunar librations, if on file, are put into pv[10][k] if
    ///              list[11] is 1 or 2.
    ///
    ///        nut   dp 4-word array that will contain nutations and rates,
    ///              depending on the setting of list[10].  the order of
    ///              quantities in nut is:
    ///
    ///                       d psi  (nutation in longitude)
    ///                       d epsilon (nutation in obliquity)
    ///                       d psi dot
    ///                       d epsilon dot
    /// ```
    pub fn jpl_state(
        ephem: *mut ::std::os::raw::c_void,
        et: f64,
        list: *const ::std::os::raw::c_int,
        pv: *mut [f64; 6usize],
        nut: *mut f64,
        bary: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    /// ```text
    /// This subroutine reads the jpl planetary ephemeris
    /// and gives the position and velocity of the point 'ntarg'
    /// with respect to 'ncent'.
    ///
    /// Calling sequence parameters:
    ///
    ///   et = (double) julian ephemeris date at which interpolation
    ///        is wanted.
    ///
    /// ntarg = integer number of 'target' point.
    ///
    /// ncent = integer number of center point.
    ///
    /// The numbering convention for 'ntarg' and 'ncent' is:
    ///
    ///         1 = mercury           8 = neptune
    ///         2 = venus             9 = pluto
    ///         3 = earth            10 = moon
    ///         4 = mars             11 = sun
    ///         5 = jupiter          12 = solar-system barycenter
    ///         6 = saturn           13 = earth-moon barycenter
    ///         7 = uranus           14 = nutations (longitude and obliq)
    ///                              15 = librations, if on eph. file
    ///                              16 = lunar mantle omega_x,omega_y,omega_z
    ///                              17 = TT-TDB, if on eph. file
    ///
    ///         (If nutations are wanted, set ntarg = 14.
    ///          For librations, set ntarg = 15. set ncent= 0.
    ///          For TT-TDB,  set ntarg = 17.  I've not actually
    ///          seen an ntarg = 16 case yet.)
    ///
    ///  rrd = output 6-element, double array of position and velocity
    ///        of point 'ntarg' relative to 'ncent'. The units are au and
    ///        au/day. For librations the units are radians and radians
    ///        per day. In the case of nutations the first four words of
    ///        rrd will be set to nutations and rates, having units of
    ///        radians and radians/day.
    ///
    ///        The option is available to have the units in km and km/sec.
    ///        for this, set km=TRUE at the beginning of the program.
    ///
    ///  calc_velocity = integer flag;  if nonzero,  velocities will be
    ///        computed,  otherwise not.
    /// ```
    pub fn jpl_pleph(
        ephem: *mut ::std::os::raw::c_void,
        et: f64,
        ntarg: ::std::os::raw::c_int,
        ncent: ::std::os::raw::c_int,
        rrd: *mut f64,
        calc_velocity: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn jpl_get_double(
        ephem: *const ::std::os::raw::c_void,
        value: ::std::os::raw::c_int,
    ) -> f64;

    pub fn jpl_get_long(
        ephem: *const ::std::os::raw::c_void,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;

    pub fn jpl_get_constant(
        idx: ::std::os::raw::c_int,
        ephem: *mut ::std::os::raw::c_void,
        constant_name: *mut ::std::os::raw::c_char,
    ) -> f64;

    pub fn jpl_init_error_code() -> ::std::os::raw::c_int;
}

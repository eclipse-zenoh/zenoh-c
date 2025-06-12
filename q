   Compiling zenoh-c v1.4.0 (/Users/milyin/ZS/zenoh-c)
error: failed to run custom build command for `zenoh-c v1.4.0 (/Users/milyin/ZS/zenoh-c)`

Caused by:
  process didn't exit successfully: `/Users/milyin/ZS/zenoh-c/target/debug/build/zenoh-c-f6e2bf7e4d532691/build-script-build` (exit status: 101)
  --- stderr
  [buildrs/opaque_types_generator.rs:117:5] feature_args.clone() = [
      "-F",
      "panic",
      "-F",
      "auth_pubkey",
      "-F",
      "auth_usrpwd",
      "-F",
      "shared-memory",
      "-F",
      "transport_multilink",
      "-F",
      "transport_quic",
      "-F",
      "transport_serial",
      "-F",
      "transport_tcp",
      "-F",
      "transport_tls",
      "-F",
      "transport_udp",
      "-F",
      "transport_unixpipe",
      "-F",
      "transport_unixsock-stream",
      "-F",
      "transport_vsock",
      "-F",
      "transport_ws",
      "-F",
      "unstable",
  ]
  [buildrs/opaque_types_generator.rs:118:5] linker_args.clone() = []

  thread 'main' panicked at buildrs/opaque_types_generator.rs:82:9:
  Failed to generate opaque types: there are 97 errors in the input data, but only 91 of them were processed as information about opaque types
  Command executed: cargo build -F panic -F auth_pubkey -F auth_usrpwd -F shared-memory -F transport_multilink -F transport_quic -F transport_serial -F transport_tcp -F transport_tls -F transport_udp -F transport_unixpipe -F transport_unixsock-stream -F transport_vsock -F transport_ws -F unstable --target aarch64-apple-darwin --manifest-path ./build-resources/opaque-types/Cargo.toml

              Compiler output:
     Compiling opaque-types v0.1.0 (/Users/milyin/ZS/zenoh-c/build-resources/opaque-types)
  error[E0432]: unresolved imports `zenoh::shm::DynamicProtocolID`, `zenoh::shm::StaticProtocolID`, `zenoh::shm::POSIX_PROTOCOL_ID`
    --> src/lib.rs:33:5
     |
  33 |     shm::DynamicProtocolID, shm::MemoryLayout, shm::PosixShmProviderBackend, shm::ProtocolID,
     |     ^^^^^^^^^^^^^^^^^^^^^^ no `DynamicProtocolID` in `shm`
  34 |     shm::ShmClient, shm::ShmClientStorage, shm::ShmProvider, shm::ShmProviderBackend,
  35 |     shm::StaticProtocolID, shm::ZLayoutError, shm::ZShm, shm::ZShmMut, shm::POSIX_PROTOCOL_ID,
     |     ^^^^^^^^^^^^^^^^^^^^^ no `StaticProtocolID` in `shm`               ^^^^^^^^^^^^^^^^^^^^^^ no `POSIX_PROTOCOL_ID` in `shm`

  error[E0107]: struct takes 1 generic argument but 2 generic arguments were supplied
     --> src/lib.rs:447:25
      |
  447 | type PosixSHMProvider = ShmProvider<StaticProtocolID<POSIX_PROTOCOL_ID>, PosixShmProviderBackend>;
      |                         ^^^^^^^^^^^ expected 1 generic argument        ------------------------- help: remove the unnecessary generic argument
      |
  note: struct defined here, with 1 generic parameter: `Backend`
     --> /Users/milyin/.cargo/git/checkouts/zenoh-9c599d5ef3e0480e/b477adf/commons/zenoh-shm/src/api/provider/shm_provider.rs:698:12
      |
  698 | pub struct ShmProvider<Backend>
      |            ^^^^^^^^^^^ -------

  error[E0107]: struct takes 1 generic argument but 2 generic arguments were supplied
     --> src/lib.rs:444:25
      |
  444 | type DummySHMProvider = ShmProvider<DynamicProtocolID, DummySHMProviderBackend>;
      |                         ^^^^^^^^^^^                  ------------------------- help: remove the unnecessary generic argument
      |                         |
      |                         expected 1 generic argument
      |
  note: struct defined here, with 1 generic parameter: `Backend`
     --> /Users/milyin/.cargo/git/checkouts/zenoh-9c599d5ef3e0480e/b477adf/commons/zenoh-shm/src/api/provider/shm_provider.rs:698:12
      |
  698 | pub struct ShmProvider<Backend>
      |            ^^^^^^^^^^^ -------

  error[E0107]: struct takes 1 generic argument but 2 generic arguments were supplied
     --> src/lib.rs:467:5
      |
  467 |     AllocLayout<'static, StaticProtocolID<POSIX_PROTOCOL_ID>, PosixShmProviderBackend>;
      |     ^^^^^^^^^^^ expected 1 generic argument                 ------------------------- help: remove the unnecessary generic argument
      |
  note: struct defined here, with 1 generic parameter: `Backend`
     --> /Users/milyin/.cargo/git/checkouts/zenoh-9c599d5ef3e0480e/b477adf/commons/zenoh-shm/src/api/provider/shm_provider.rs:145:12
      |
  145 | pub struct AllocLayout<'a, Backend>
      |            ^^^^^^^^^^^     -------

  error[E0107]: struct takes 1 generic argument but 2 generic arguments were supplied
     --> src/lib.rs:470:32
      |
  470 | type DummyDynamicAllocLayout = AllocLayout<'static, DynamicProtocolID, DummySHMProviderBackend>;
      |                                ^^^^^^^^^^^                           ------------------------- help: remove the unnecessary generic argument
      |                                |
      |                                expected 1 generic argument
      |
  note: struct defined here, with 1 generic parameter: `Backend`
     --> /Users/milyin/.cargo/git/checkouts/zenoh-9c599d5ef3e0480e/b477adf/commons/zenoh-shm/src/api/provider/shm_provider.rs:145:12
      |
  145 | pub struct AllocLayout<'a, Backend>
      |            ^^^^^^^^^^^     -------

  error[E0277]: the trait bound `DummySHMProviderBackend: zenoh_shm::api::common::with_id::WithProtocolID` is not satisfied
     --> src/lib.rs:421:29
      |
  421 | impl ShmProviderBackend for DummySHMProviderBackend {
      |                             ^^^^^^^^^^^^^^^^^^^^^^^ the trait `zenoh_shm::api::common::with_id::WithProtocolID` is not implemented for `DummySHMProviderBackend`
      |
      = help: the following other types implement trait `zenoh_shm::api::common::with_id::WithProtocolID`:
                PosixShmClient
                PosixShmProviderBackend
  note: required by a bound in `ShmProviderBackend`
     --> /Users/milyin/.cargo/git/checkouts/zenoh-9c599d5ef3e0480e/b477adf/commons/zenoh-shm/src/api/provider/shm_provider_backend.rs:23:31
      |
  23  | pub trait ShmProviderBackend: WithProtocolID {
      |                               ^^^^^^^^^^^^^^ required by this bound in `ShmProviderBackend`
      = note: `ShmProviderBackend` is a "sealed trait", because to implement it you also need to implement `zenoh_shm::api::common::with_id::WithProtocolID`, which is not accessible; this is usually done to force you to use one of the provided types that already implement it
      = help: the following types implement the trait:
                zenoh::shm::PosixShmClient
                zenoh::shm::PosixShmProviderBackend

  error[E0080]: evaluation of constant value failed
    --> src/lib.rs:57:1
     |
  57 | get_opaque_type_data!(ZBytes, z_owned_bytes_t);
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_bytes_t, align: 8, size: 40', src/lib.rs:57:1
     |
     = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
    --> src/lib.rs:59:1
     |
  59 | get_opaque_type_data!(ZBytes, z_loaned_bytes_t);
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_bytes_t, align: 8, size: 40', src/lib.rs:59:1
     |
     = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
    --> src/lib.rs:68:1
     |
  68 | get_opaque_type_data!(CSlice, z_owned_slice_t);
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_slice_t, align: 8, size: 32', src/lib.rs:68:1
     |
     = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
    --> src/lib.rs:70:1
     |
  70 | get_opaque_type_data!(CSlice, z_view_slice_t);
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_view_slice_t, align: 8, size: 32', src/lib.rs:70:1
     |
     = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
    --> src/lib.rs:72:1
     |
  72 | get_opaque_type_data!(CSlice, z_loaned_slice_t);
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_slice_t, align: 8, size: 32', src/lib.rs:72:1
     |
     = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
    --> src/lib.rs:75:1
     |
  75 | get_opaque_type_data!(CSlice, z_owned_string_t);
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_string_t, align: 8, size: 32', src/lib.rs:75:1
     |
     = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
    --> src/lib.rs:77:1
     |
  77 | get_opaque_type_data!(CSlice, z_view_string_t);
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_view_string_t, align: 8, size: 32', src/lib.rs:77:1
     |
     = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
    --> src/lib.rs:79:1
     |
  79 | get_opaque_type_data!(CSlice, z_loaned_string_t);
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_string_t, align: 8, size: 32', src/lib.rs:79:1
     |
     = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
    --> src/lib.rs:83:1
     |
  83 | get_opaque_type_data!(Vec<CSlice>, z_owned_string_array_t);
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_string_array_t, align: 8, size: 24', src/lib.rs:83:1
     |
     = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
    --> src/lib.rs:85:1
     |
  85 | get_opaque_type_data!(Vec<CSlice>, z_loaned_string_array_t);
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_string_array_t, align: 8, size: 24', src/lib.rs:85:1
     |
     = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
    --> src/lib.rs:91:1
     |
  91 | get_opaque_type_data!(Option<Sample>, z_owned_sample_t);
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_sample_t, align: 8, size: 232', src/lib.rs:91:1
     |
     = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
    --> src/lib.rs:93:1
     |
  93 | get_opaque_type_data!(Sample, z_loaned_sample_t);
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_sample_t, align: 8, size: 232', src/lib.rs:93:1
     |
     = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
    --> src/lib.rs:96:1
     |
  96 | get_opaque_type_data!(ZBytesReader<'static>, z_bytes_reader_t);
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_bytes_reader_t, align: 8, size: 24', src/lib.rs:96:1
     |
     = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
    --> src/lib.rs:99:1
     |
  99 | get_opaque_type_data!(Option<ZBytesWriter>, z_owned_bytes_writer_t);
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_bytes_writer_t, align: 8, size: 64', src/lib.rs:99:1
     |
     = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:101:1
      |
  101 | get_opaque_type_data!(ZBytesWriter, z_loaned_bytes_writer_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_bytes_writer_t, align: 8, size: 64', src/lib.rs:101:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:104:1
      |
  104 | get_opaque_type_data!(ZBytesSliceIterator<'static>, z_bytes_slice_iterator_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_bytes_slice_iterator_t, align: 8, size: 24', src/lib.rs:104:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:107:1
      |
  107 | get_opaque_type_data!(Encoding, z_owned_encoding_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_encoding_t, align: 8, size: 48', src/lib.rs:107:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:109:1
      |
  109 | get_opaque_type_data!(Encoding, z_loaned_encoding_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_encoding_t, align: 8, size: 48', src/lib.rs:109:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:112:1
      |
  112 | get_opaque_type_data!(Option<Reply>, z_owned_reply_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_reply_t, align: 8, size: 256', src/lib.rs:112:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:114:1
      |
  114 | get_opaque_type_data!(Reply, z_loaned_reply_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_reply_t, align: 8, size: 256', src/lib.rs:114:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:117:1
      |
  117 | get_opaque_type_data!(ReplyError, z_owned_reply_err_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_reply_err_t, align: 8, size: 88', src/lib.rs:117:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:119:1
      |
  119 | get_opaque_type_data!(ReplyError, z_loaned_reply_err_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_reply_err_t, align: 8, size: 88', src/lib.rs:119:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:124:1
      |
  124 | get_opaque_type_data!(Option<Query>, z_owned_query_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_query_t, align: 8, size: 144', src/lib.rs:124:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:126:1
      |
  126 | get_opaque_type_data!(Query, z_loaned_query_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_query_t, align: 8, size: 144', src/lib.rs:126:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:131:1
      |
  131 | get_opaque_type_data!(Option<Queryable<()>>, z_owned_queryable_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_queryable_t, align: 8, size: 48', src/lib.rs:131:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:133:1
      |
  133 | get_opaque_type_data!(Queryable<()>, z_loaned_queryable_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_queryable_t, align: 8, size: 48', src/lib.rs:133:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:139:1
      |
  139 | get_opaque_type_data!(Option<Querier>, z_owned_querier_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_querier_t, align: 8, size: 80', src/lib.rs:139:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:142:1
      |
  142 | get_opaque_type_data!(Querier, z_loaned_querier_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_querier_t, align: 8, size: 80', src/lib.rs:142:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:150:1
      |
  150 | / get_opaque_type_data!(
  151 | |     Option<(zenoh_ext::FetchingSubscriber<()>, &'static Session)>,
  152 | |     ze_owned_querying_subscriber_t
  153 | | );
      | |_^ the evaluated program panicked at 'type: ze_owned_querying_subscriber_t, align: 8, size: 80', src/lib.rs:150:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:157:1
      |
  157 | / get_opaque_type_data!(
  158 | |     (zenoh_ext::FetchingSubscriber<()>, &'static Session),
  159 | |     ze_loaned_querying_subscriber_t
  160 | | );
      | |_^ the evaluated program panicked at 'type: ze_loaned_querying_subscriber_t, align: 8, size: 80', src/lib.rs:157:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:168:1
      |
  168 | / get_opaque_type_data!(
  169 | |     Option<zenoh_ext::AdvancedSubscriber<()>>,
  170 | |     ze_owned_advanced_subscriber_t
  171 | | );
      | |_^ the evaluated program panicked at 'type: ze_owned_advanced_subscriber_t, align: 8, size: 152', src/lib.rs:168:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:175:1
      |
  175 | / get_opaque_type_data!(
  176 | |     zenoh_ext::AdvancedSubscriber<()>,
  177 | |     ze_loaned_advanced_subscriber_t
  178 | | );
      | |_^ the evaluated program panicked at 'type: ze_loaned_advanced_subscriber_t, align: 8, size: 152', src/lib.rs:175:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:185:1
      |
  185 | / get_opaque_type_data!(
  186 | |     Option<zenoh_ext::SampleMissListener<()>>,
  187 | |     ze_owned_sample_miss_listener_t
  188 | | );
      | |_^ the evaluated program panicked at 'type: ze_owned_sample_miss_listener_t, align: 8, size: 24', src/lib.rs:185:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:196:1
      |
  196 | / get_opaque_type_data!(
  197 | |     Option<zenoh_ext::AdvancedPublisher<'static>>,
  198 | |     ze_owned_advanced_publisher_t
  199 | | );
      | |_^ the evaluated program panicked at 'type: ze_owned_advanced_publisher_t, align: 8, size: 232', src/lib.rs:196:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:203:1
      |
  203 | / get_opaque_type_data!(
  204 | |     zenoh_ext::AdvancedPublisher<'static>,
  205 | |     ze_loaned_advanced_publisher_t
  206 | | );
      | |_^ the evaluated program panicked at 'type: ze_loaned_advanced_publisher_t, align: 8, size: 232', src/lib.rs:203:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:222:1
      |
  222 | get_opaque_type_data!(Option<KeyExpr<'static>>, z_owned_keyexpr_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_keyexpr_t, align: 8, size: 32', src/lib.rs:222:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:224:1
      |
  224 | get_opaque_type_data!(Option<KeyExpr<'static>>, z_view_keyexpr_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_view_keyexpr_t, align: 8, size: 32', src/lib.rs:224:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:236:1
      |
  236 | get_opaque_type_data!(KeyExpr<'static>, z_loaned_keyexpr_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_keyexpr_t, align: 8, size: 32', src/lib.rs:236:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:239:1
      |
  239 | get_opaque_type_data!(Option<Session>, z_owned_session_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_session_t, align: 8, size: 8', src/lib.rs:239:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:241:1
      |
  241 | get_opaque_type_data!(Session, z_loaned_session_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_session_t, align: 8, size: 8', src/lib.rs:241:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:245:1
      |
  245 | / get_opaque_type_data!(
  246 | |     Option<NolocalJoinHandle<zenoh::Result<()>>>,
  247 | |     zc_owned_concurrent_close_handle_t
  248 | | );
      | |_^ the evaluated program panicked at 'type: zc_owned_concurrent_close_handle_t, align: 8, size: 16', src/lib.rs:245:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:251:1
      |
  251 | get_opaque_type_data!(Option<Config>, z_owned_config_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_config_t, align: 8, size: 1960', src/lib.rs:251:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:253:1
      |
  253 | get_opaque_type_data!(Config, z_loaned_config_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_config_t, align: 8, size: 1960', src/lib.rs:253:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:258:1
      |
  258 | get_opaque_type_data!(ZenohId, z_id_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_id_t, align: 1, size: 16', src/lib.rs:258:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:263:1
      |
  263 | get_opaque_type_data!(Timestamp, z_timestamp_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_timestamp_t, align: 8, size: 24', src/lib.rs:263:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:266:1
      |
  266 | get_opaque_type_data!(Option<Publisher<'static>>, z_owned_publisher_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_publisher_t, align: 8, size: 112', src/lib.rs:266:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:268:1
      |
  268 | get_opaque_type_data!(Publisher<'static>, z_loaned_publisher_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_publisher_t, align: 8, size: 112', src/lib.rs:268:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:276:1
      |
  276 | get_opaque_type_data!(Option<MatchingListener<()>>, z_owned_matching_listener_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_matching_listener_t, align: 8, size: 24', src/lib.rs:276:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:282:1
      |
  282 | get_opaque_type_data!(Option<Subscriber<()>>, z_owned_subscriber_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_subscriber_t, align: 8, size: 48', src/lib.rs:282:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:284:1
      |
  284 | get_opaque_type_data!(Subscriber<()>, z_loaned_subscriber_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_subscriber_t, align: 8, size: 48', src/lib.rs:284:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:291:1
      |
  291 | get_opaque_type_data!(Option<LivelinessToken>, z_owned_liveliness_token_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_liveliness_token_t, align: 8, size: 16', src/lib.rs:291:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:292:1
      |
  292 | get_opaque_type_data!(LivelinessToken, z_loaned_liveliness_token_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_liveliness_token_t, align: 8, size: 16', src/lib.rs:292:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:300:1
      |
  300 | / get_opaque_type_data!(
  301 | |     Option<zenoh_ext::PublicationCache>,
  302 | |     ze_owned_publication_cache_t
  303 | | );
      | |_^ the evaluated program panicked at 'type: ze_owned_publication_cache_t, align: 8, size: 128', src/lib.rs:300:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:307:1
      |
  307 | get_opaque_type_data!(zenoh_ext::PublicationCache, ze_loaned_publication_cache_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: ze_loaned_publication_cache_t, align: 8, size: 128', src/lib.rs:307:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:310:1
      |
  310 | / get_opaque_type_data!(
  311 | |     Option<(Mutex<()>, Option<MutexGuard<'static, ()>>)>,
  312 | |     z_owned_mutex_t
  313 | | );
      | |_^ the evaluated program panicked at 'type: z_owned_mutex_t, align: 8, size: 32', src/lib.rs:310:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:315:1
      |
  315 | / get_opaque_type_data!(
  316 | |     (Mutex<()>, Option<MutexGuard<'static, ()>>),
  317 | |     z_loaned_mutex_t
  318 | | );
      | |_^ the evaluated program panicked at 'type: z_loaned_mutex_t, align: 8, size: 32', src/lib.rs:315:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:323:1
      |
  323 | get_opaque_type_data!(Option<Condvar>, z_owned_condvar_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_condvar_t, align: 8, size: 24', src/lib.rs:323:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:325:1
      |
  325 | get_opaque_type_data!(Condvar, z_loaned_condvar_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_condvar_t, align: 8, size: 16', src/lib.rs:325:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:328:1
      |
  328 | get_opaque_type_data!(Option<JoinHandle<()>>, z_owned_task_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_task_t, align: 8, size: 32', src/lib.rs:328:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:331:1
      |
  331 | get_opaque_type_data!(Option<Hello>, z_owned_hello_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_hello_t, align: 8, size: 48', src/lib.rs:331:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:333:1
      |
  333 | get_opaque_type_data!(Hello, z_loaned_hello_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_hello_t, align: 8, size: 48', src/lib.rs:333:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:338:1
      |
  338 | get_opaque_type_data!(Option<Arc<dyn ShmClient>>, z_owned_shm_client_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_shm_client_t, align: 8, size: 16', src/lib.rs:338:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:342:1
      |
  342 | / get_opaque_type_data!(
  343 | |     Option<Vec<(ProtocolID, Arc<dyn ShmClient>)>>,
  344 | |     zc_owned_shm_client_list_t
  345 | | );
      | |_^ the evaluated program panicked at 'type: zc_owned_shm_client_list_t, align: 8, size: 24', src/lib.rs:342:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:349:1
      |
  349 | / get_opaque_type_data!(
  350 | |     Vec<(ProtocolID, Arc<dyn ShmClient>)>,
  351 | |     zc_loaned_shm_client_list_t
  352 | | );
      | |_^ the evaluated program panicked at 'type: zc_loaned_shm_client_list_t, align: 8, size: 24', src/lib.rs:349:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:357:1
      |
  357 | get_opaque_type_data!(Option<Arc<ShmClientStorage>>, z_owned_shm_client_storage_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_shm_client_storage_t, align: 8, size: 8', src/lib.rs:357:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:360:1
      |
  360 | get_opaque_type_data!(Arc<ShmClientStorage>, z_loaned_shm_client_storage_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_shm_client_storage_t, align: 8, size: 8', src/lib.rs:360:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:365:1
      |
  365 | get_opaque_type_data!(Option<MemoryLayout>, z_owned_memory_layout_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_memory_layout_t, align: 8, size: 16', src/lib.rs:365:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:369:1
      |
  369 | get_opaque_type_data!(MemoryLayout, z_loaned_memory_layout_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_memory_layout_t, align: 8, size: 16', src/lib.rs:369:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:374:1
      |
  374 | get_opaque_type_data!(Option<ChunkAllocResult>, z_owned_chunk_alloc_result_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_chunk_alloc_result_t, align: 8, size: 48', src/lib.rs:374:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:379:1
      |
  379 | get_opaque_type_data!(Option<ZShm>, z_owned_shm_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_shm_t, align: 8, size: 80', src/lib.rs:379:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:383:1
      |
  383 | get_opaque_type_data!(zshm, z_loaned_shm_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_shm_t, align: 8, size: 80', src/lib.rs:383:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:388:1
      |
  388 | get_opaque_type_data!(Option<ZShmMut>, z_owned_shm_mut_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_shm_mut_t, align: 8, size: 80', src/lib.rs:388:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:392:1
      |
  392 | get_opaque_type_data!(zshmmut, z_loaned_shm_mut_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_shm_mut_t, align: 8, size: 80', src/lib.rs:392:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:488:1
      |
  488 | / get_opaque_type_data!(
  489 | |     Option<FifoChannelHandler<Sample>>,
  490 | |     z_owned_fifo_handler_sample_t
  491 | | );
      | |_^ the evaluated program panicked at 'type: z_owned_fifo_handler_sample_t, align: 8, size: 8', src/lib.rs:488:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:493:1
      |
  493 | get_opaque_type_data!(FifoChannelHandler<Sample>, z_loaned_fifo_handler_sample_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_fifo_handler_sample_t, align: 8, size: 8', src/lib.rs:493:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:496:1
      |
  496 | / get_opaque_type_data!(
  497 | |     Option<RingChannelHandler<Sample>>,
  498 | |     z_owned_ring_handler_sample_t
  499 | | );
      | |_^ the evaluated program panicked at 'type: z_owned_ring_handler_sample_t, align: 8, size: 8', src/lib.rs:496:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:501:1
      |
  501 | get_opaque_type_data!(RingChannelHandler<Sample>, z_loaned_ring_handler_sample_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_ring_handler_sample_t, align: 8, size: 8', src/lib.rs:501:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:504:1
      |
  504 | / get_opaque_type_data!(
  505 | |     Option<FifoChannelHandler<Query>>,
  506 | |     z_owned_fifo_handler_query_t
  507 | | );
      | |_^ the evaluated program panicked at 'type: z_owned_fifo_handler_query_t, align: 8, size: 8', src/lib.rs:504:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:509:1
      |
  509 | get_opaque_type_data!(FifoChannelHandler<Query>, z_loaned_fifo_handler_query_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_fifo_handler_query_t, align: 8, size: 8', src/lib.rs:509:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:512:1
      |
  512 | / get_opaque_type_data!(
  513 | |     Option<RingChannelHandler<Query>>,
  514 | |     z_owned_ring_handler_query_t
  515 | | );
      | |_^ the evaluated program panicked at 'type: z_owned_ring_handler_query_t, align: 8, size: 8', src/lib.rs:512:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:517:1
      |
  517 | get_opaque_type_data!(RingChannelHandler<Query>, z_loaned_ring_handler_query_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_ring_handler_query_t, align: 8, size: 8', src/lib.rs:517:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:520:1
      |
  520 | / get_opaque_type_data!(
  521 | |     Option<FifoChannelHandler<Reply>>,
  522 | |     z_owned_fifo_handler_reply_t
  523 | | );
      | |_^ the evaluated program panicked at 'type: z_owned_fifo_handler_reply_t, align: 8, size: 8', src/lib.rs:520:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:525:1
      |
  525 | get_opaque_type_data!(FifoChannelHandler<Reply>, z_loaned_fifo_handler_reply_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_fifo_handler_reply_t, align: 8, size: 8', src/lib.rs:525:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:528:1
      |
  528 | / get_opaque_type_data!(
  529 | |     Option<RingChannelHandler<Reply>>,
  530 | |     z_owned_ring_handler_reply_t
  531 | | );
      | |_^ the evaluated program panicked at 'type: z_owned_ring_handler_reply_t, align: 8, size: 8', src/lib.rs:528:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:533:1
      |
  533 | get_opaque_type_data!(RingChannelHandler<Reply>, z_loaned_ring_handler_reply_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_ring_handler_reply_t, align: 8, size: 8', src/lib.rs:533:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:538:1
      |
  538 | get_opaque_type_data!(SourceInfo, z_owned_source_info_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_owned_source_info_t, align: 4, size: 32', src/lib.rs:538:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:542:1
      |
  542 | get_opaque_type_data!(SourceInfo, z_loaned_source_info_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_loaned_source_info_t, align: 4, size: 32', src/lib.rs:542:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:546:1
      |
  546 | get_opaque_type_data!(EntityGlobalId, z_entity_global_id_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: z_entity_global_id_t, align: 4, size: 20', src/lib.rs:546:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:549:1
      |
  549 | get_opaque_type_data!(Option<zenoh_ext::ZSerializer>, ze_owned_serializer_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: ze_owned_serializer_t, align: 8, size: 64', src/lib.rs:549:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:551:1
      |
  551 | get_opaque_type_data!(zenoh_ext::ZSerializer, ze_loaned_serializer_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: ze_loaned_serializer_t, align: 8, size: 64', src/lib.rs:551:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  error[E0080]: evaluation of constant value failed
     --> src/lib.rs:553:1
      |
  553 | get_opaque_type_data!(zenoh_ext::ZDeserializer<'static>, ze_deserializer_t);
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'type: ze_deserializer_t, align: 8, size: 24', src/lib.rs:553:1
      |
      = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `get_opaque_type_data` (in Nightly builds, run with -Z macro-backtrace for more info)

  Some errors have detailed explanations: E0080, E0107, E0277, E0432.
  For more information about an error, try `rustc --explain E0080`.
  error: could not compile `opaque-types` (lib) due to 97 previous errors

  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

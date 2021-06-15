fn main() {
    windows::build!(
        Windows::Win32::Foundation:: {
            NTSTATUS,
            PSTR,
            PWSTR,
            S_OK,
        },
        Windows::Win32::System::Com:: {
            CoCreateGuid,
        },
    )
}
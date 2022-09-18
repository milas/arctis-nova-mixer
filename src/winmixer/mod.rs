use std::error::Error;
use windows::System::Diagnostics::ProcessDiagnosticInfo;
use windows::Win32::System::Com::StructuredStorage::PROPVARIANT;
use windows::core::{GUID,Interface};
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use windows::Win32::Media::Audio::*;
use windows::Win32::System::Com::{CLSCTX_ALL, CoInitialize, CoUninitialize, CLSCTX};
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER};

pub unsafe fn mixer() -> Result<(), Box<dyn Error>> {
    CoInitialize(std::ptr::null_mut())?;

    let immde: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
    let imm = immde.GetDefaultAudioEndpoint(EDataFlow(0 /*eRender*/), ERole(1 /*multimedia*/))?;
    let iae: IAudioEndpointVolume = imm.Activate(
        CLSCTX_INPROC_SERVER,
        &PROPVARIANT::default(),
    )?;

    iae.SetMasterVolumeLevelScalar(0.6, &GUID::default())?;

    let iasm: IAudioSessionManager2 = imm.Activate(CLSCTX::default(), &PROPVARIANT::default())?;
    let se = iasm.GetSessionEnumerator()?;
    println!("{}", se.GetCount()?);
    for i in 0..se.GetCount()? {
        let ctl1: IAudioSessionControl = se.GetSession(i)?;
        let ctl: IAudioSessionControl2 = ctl1.cast()?;

        let vol: ISimpleAudioVolume = ctl.cast()?;

        println!("{}: {} @ {}", ctl.GetDisplayName()?.display(), ctl.GetProcessId()?, vol.GetMasterVolume()?);
        if ctl.GetProcessId()? == 0 {
            let guid = GUID::zeroed();
            vol.SetMasterVolume(0.2, &guid)?;
        }
    }

    for process in ProcessDiagnosticInfo::GetForProcesses()?.into_iter() {
        println!("{}: {}", process.ProcessId()?, process.ExecutableFileName()?);
    }

    CoUninitialize();

    Ok(())
}
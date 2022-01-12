
pub mod primitive;
pub mod mmu;
pub mod emulator;

use std::io::{self, Write};
use emulator::{Emulator, Register, VmExit};
use mmu::{VirtAddr, Perm, Sections, PERM_READ, PERM_WRITE, PERM_EXEC};
use rand::Rng;
use std::time::SystemTime;

const  VERBOSE_PRINTS: bool = true;

const KEYS: [&str; 512] = ["d5AC", "e69L", "KcPc", "HU3e", "6NI0", "cRz7", "acXW", "VohW", "pSoA", "HaVm", "UOcl", "toyr", "1PUy", "KbLV", "QPa9", "MsKa", "JqjA", "pKSY", "qvgr", "klAx", "YOnD", "gnjr", "TEUb", "1ijD", "QCGA", "Ymuc", "8x03", "MxZw", "wOlL", "bOVx", "1Ff3", "CNxp", "5Ea4", "h6Aa", "zaiu", "B0If", "bDfB", "cDXi", "7B0e", "PWdj", "h264", "SUJP", "jpRN", "90Yx", "Ur1E", "TDFR", "2hd7", "WZ3P", "XEGc", "b0rK", "EV6Y", "Xozd", "rwf7", "Ca6u", "qBSS", "RptZ", "qBEJ", "Vxwz", "Q7Ah", "C3iT", "GWY8", "mGus", "ZJj1", "cffK", "Ygdn", "VuVR", "93i9", "n864", "sODV", "ns3i", "94j7", "yb3J", "nZVk", "5udm", "moLV", "6nqf", "cvW7", "gebr", "gd55", "ucQF", "hnfm", "csD9", "vNPX", "xFVK", "nD6t", "BYL2", "kL9W", "psZT", "yhtG", "Xy7u", "BG6U", "YjJG", "0yIH", "wFPe", "NRKZ", "PGdc", "60pf", "rjmr", "dJE5", "9a7E", "qjbe", "rxDS", "6LTT", "sa7q", "M8xy", "iPP4", "nT2u", "bFox", "1LJ0", "gWbL", "mYhZ", "vJEv", "af5G", "E8dh", "wKzg", "XWLZ", "sP7X", "LJNS", "2s7n", "LRUt", "eM5R", "zt2A", "Uoy3", "uxuK", "Avkb", "z8wI", "0XU4", "AtOL", "pArd", "Tlas", "HYU9", "Dqrf", "2ozi", "6mMG", "HeoP", "ZUWi", "Dvsg", "MBAr", "aK1p", "IM1z", "m7XL", "KHjZ", "4H08", "qlQT", "RqhL", "X5yO", "s3U8", "7TAk", "wEpE", "pfqk", "kiTw", "3lLA", "NWQE", "QVhw", "D75N", "dReF", "xVkj", "q3RE", "FeWM", "3chw", "F24T", "GTFQ", "K5UE", "36nY", "Vc7z", "h7cQ", "o3CR", "AcQu", "MYid", "520g", "H94s", "M6Be", "WcOq", "B2CL", "NUzA", "NjCf", "2K7J", "hm3F", "8MrO", "B1u6", "kq2u", "PJnV", "WkyA", "7FTC", "NFH9", "CpBX", "QUMG", "keiX", "TFBP", "veLp", "ZK4N", "RGvI", "nO1v", "jGkP", "T6pM", "um0F", "Ix8F", "ljM2", "rUxy", "FFlD", "nKLw", "EoGC", "17yV", "42u4", "ZCjw", "rn8h", "YcUA", "wSql", "Xjo4", "vKSx", "THvt", "nW2X", "DVCp", "Dkir", "o9UD", "Jz9j", "kZQ6", "dz8d", "bj7a", "mySe", "6870", "bTua", "L202", "0MCL", "xqa3", "DpTh", "xLUd", "JuGb", "WhPY", "piSw", "um7v", "7DEo", "JI4b", "JXhp", "4TA4", "4ECy", "ZoSX", "lov5", "C785", "ge2m", "DzxO", "IZBv", "6lAh", "AZ0a", "Mi5p", "OV5A", "3SsQ", "rRfU", "oZKj", "0lDe", "7rcq", "vwuE", "H3ln", "rBKI", "t1Rf", "s9L9", "QIfn", "2vsH", "V3HV", "lmXB", "huon", "Q4IY", "hy6i", "rm21", "ogqu", "iSb6", "M7sb", "WsXC", "JRvZ", "kjmS", "JY7A", "c4hh", "ttUx", "OUcv", "59Ja", "XUNU", "Y2MT", "6MaP", "4mep", "Ai8q", "e14R", "XNVW", "1v38", "2zyI", "gzUA", "ZduW", "YXbD", "HOhR", "ln6I", "gDry", "aSmS", "PBRz", "64eX", "4Jco", "Ymgk", "vElD", "C90F", "cLab", "V5rX", "vW7e", "tUx3", "fNuD", "UQsu", "fZCy", "xGhT", "IRwV", "cWvl", "EYEa", "wCQg", "d1CY", "lC7c", "pv4r", "JQpc", "GR4J", "dHQc", "ZA4i", "ZBxs", "yBdf", "6Pgw", "7nOf", "W5Oc", "Upte", "yqGE", "sM4y", "nwtt", "cqTh", "TqQU", "XeN3", "WA83", "7N5W", "gkv2", "cyFq", "tP6m", "xcIy", "oeeF", "6LHI", "YIgN", "WS5W", "dsj9", "Sz6r", "tcTB", "c30m", "35ez", "ZYXC", "VfRm", "UyOX", "z6Pa", "QO8I", "VigP", "cun5", "1MYk", "YzTb", "6d1P", "uLMT", "LMdi", "HcdH", "NKLD", "frgu", "32l2", "magz", "gogz", "CwpC", "FHKZ", "bwEm", "VES2", "Zwma", "OhQU", "uD5z", "VOv1", "4fbj", "PZ38", "Tfyk", "Sj6v", "M9zx", "cyoO", "TrwZ", "pKLm", "AzuH", "oM8o", "wVcj", "4tl4", "mPT2", "PlM1", "w8H5", "RjrX", "I2QH", "nOTy", "AMdM", "4AEm", "jCBJ", "ChVM", "Kxud", "U18t", "SjIr", "ODyD", "TQBq", "WIZe", "SGQm", "YzVl", "UD84", "havE", "FP1W", "bRSm", "RWfh", "Anu4", "sDbK", "RUuU", "9QAR", "cc40", "ZYq3", "zC29", "2K1a", "FT6R", "ipuU", "gNT7", "1wQl", "U3sU", "wbVn", "KRfy", "oZWT", "YeR7", "nkdT", "Djpm", "UhhX", "m01P", "BiY6", "pS52", "YA9r", "H5ck", "icWz", "xQU5", "bVpx", "yLma", "EGqs", "KXwp", "FGvY", "GHhD", "Mf8N", "nM0v", "97k8", "fkzF", "b3Cb", "XmMh", "LvId", "KvBk", "qj9L", "DgiX", "QlXd", "TcGl", "jenX", "z20b", "qFvR", "FBmC", "2Qsj", "3CnA", "XtsU", "nBSv", "oRII", "9UjJ", "TeDg", "py3E", "EqKc", "5bh3", "xnfi", "iIqv", "oreO", "ofCL", "FDBZ", "u2Kp", "u20F", "uRFI", "FrzP", "hpzB", "5UpN", "Jde4", "6Wzd", "SAOC", "1cTw", "HxjF", "oSOS", "4hcQ", "XzZY", "5YG0", "NpLx", "YsW6", "liJB", "MFsi", "Nxht", "HeLB", "kS5d", "c7DC", "y4wp", "BOKZ", "NLXo", "VEsz", "pBw6", "dfqX", "fMa7", "dEBe", "Wode", "SSWg", "IFGz", "OhJQ", "cDEI", "8vLQ", "XAlI", "IWs5", "DnXn", "tuuj", "30Dl", "CHFO", "4NW9"];

const TIME: [u64; 512] = [4268402577, 2885444848, 6069527499, 3145231422, 435446863, 8378224368, 3228967217, 7095236857, 1345720511, 7411093226, 5415747561, 3244092716, 2857564979, 8340894259, 5904418627, 4991244969, 9317743589, 2105601567, 5924229938, 3842128170, 5309060976, 922998368, 1910387444, 1965309704, 2853839735, 8495544979, 3264424471, 242787460, 422966394, 156564325, 5821048402, 3905611929, 5385117059, 6114541570, 9880516625, 5597995322, 2913536908, 4690008599, 1198486615, 9358834522, 6404697848, 6107932877, 5384440862, 9989197857, 4492107861, 2198987387, 3709507112, 8520908854, 3564466967, 8028875357, 15778373, 9517639914, 1203375854, 5910899966, 4108525271, 1559898759, 3933338684, 7346223109, 8149842018, 6393922077, 8396709548, 2259667813, 7649712932, 2830902175, 6718596690, 6349344107, 1274985751, 2394553211, 3865061522, 8187479579, 3461809380, 6061203398, 5469121790, 761304964, 2932589548, 6913725435, 5671848956, 1865454257, 900683550, 873421101, 4476961496, 1567933721, 9532054217, 5059489056, 8924946177, 5293446683, 897771726, 6569832794, 4215658681, 7355000504, 1525596460, 8243479600, 8357194648, 1436581259, 4147831902, 7131515140, 9165645382, 3373135736, 4762688853, 7051515591, 5742151542, 5194861129, 1788190062, 2392872579, 2092227019, 9512472155, 6478723112, 1521384371, 6138533946, 4491502961, 8081465026, 4977563444, 8916808771, 8080433017, 3970467922, 757283749, 360559202, 9285892444, 9927452914, 9383632968, 7213709315, 6855962730, 8244803358, 8248662971, 3286258206, 9166996062, 7299274644, 1070572557, 4260822543, 1434776933, 9623413952, 6833511526, 4875721432, 8776642404, 4113928479, 493737736, 4578528746, 2235083029, 6183121635, 9215686052, 3356032255, 6114672613, 6936031443, 6146391712, 7541038861, 6549142557, 6177835265, 7584322712, 55221795, 7585389235, 2480372160, 9625097424, 1919675283, 895084338, 355869862, 205739262, 8717057551, 611255672, 4203867055, 8208388479, 9098060199, 996582590, 1875232886, 6750728510, 551824882, 8055448279, 86577372, 3198135717, 3991081255, 151608681, 3747310102, 3160362397, 6554830692, 4380125460, 8309927962, 5445100772, 736368206, 466950748, 6602424161, 589867036, 1568743227, 1670774675, 3150251578, 1451995315, 4032653337, 5452910581, 3291113365, 7323558298, 4123557750, 6081668477, 9924902298, 6609220296, 3293967467, 5179901414, 2217427171, 3986764999, 1366781721, 9877959478, 3376980729, 698234975, 3021484626, 4032723059, 15334493, 442561527, 2675128373, 1815405923, 4347664174, 4318413683, 9632426691, 2621378704, 8052816365, 491838597, 6407797371, 7178934265, 3221897001, 708694564, 296368735, 6860110571, 1318153637, 5536867506, 9487145141, 9875914376, 3150896148, 9107260906, 50658991, 6707815195, 5632896651, 6216352934, 9178975, 5884530902, 3743945638, 4481453072, 9446243391, 166608532, 5461204953, 3241349916, 5110227245, 674237284, 187488621, 6934425584, 6290953118, 2400946414, 8217096913, 952486898, 970525599, 488459318, 4735653134, 3145015848, 8034330075, 8595467169, 4539960303, 3735818822, 7450922811, 865122788, 7935575865, 7065323649, 1335880670, 6958977781, 6162736174, 7580508278, 5127260212, 4780413467, 7722400305, 944457970, 1129594694, 8312793716, 1935428450, 5635815361, 7964334230, 7291762929, 3156698972, 3735152988, 8697957328, 586315968, 7215358444, 3659174815, 489488213, 1910505596, 4321462907, 3708317537, 8458835547, 8528877184, 8984167680, 5126206369, 9359107284, 5297749801, 349850542, 1528133337, 4623985868, 6354397475, 1903333632, 1207194514, 739292723, 5852522879, 4675871868, 2906201271, 6231787679, 7607437181, 7132942567, 8018105381, 2095637486, 2741804157, 8453862548, 9980006175, 4818942555, 5570555361, 3734305896, 9376881637, 1185828915, 2441093202, 8927325356, 5810077875, 8293206457, 5063938648, 3631239066, 8609290806, 5085558201, 8429785445, 4482738018, 2217720933, 183455347, 4477593668, 3136906287, 3633335959, 15410070, 1436178207, 7025204146, 1370225703, 8989614971, 4691661952, 801118997, 5430577972, 1585566399, 9015191902, 7534967177, 4260629188, 4055806100, 918171659, 8899369946, 8611977206, 624320623, 6866518123, 2624474466, 9051269606, 2211054881, 133085191, 2265428392, 2355068830, 8936232187, 3405118138, 8331924460, 3040584505, 8915638559, 3907340764, 4272457273, 482827154, 8314593863, 7902783570, 6775632196, 9793038781, 2444633026, 688250236, 7849597711, 419696316, 9352749421, 1933578230, 7284571988, 482567947, 7325059293, 4508725458, 1712871426, 1823860511, 6086146493, 2495829783, 7012191722, 9746852574, 2892723578, 1394749697, 260812270, 5760792464, 6438623239, 2119056325, 1658435796, 2557702290, 6197867903, 9238080003, 2382213369, 5097457184, 4918227961, 2626508656, 2345921517, 8520018417, 8760630553, 8542381084, 246153574, 6634276734, 2147076623, 445702963, 6886813707, 9967459170, 2384049848, 2708281673, 6465283355, 8317712187, 3456044484, 4358611897, 4985485704, 4183951369, 2434287597, 6992186469, 3192676850, 7183788963, 3931709708, 2323668948, 3787556150, 7984949258, 6871899257, 4316134722, 6713823573, 9950581858, 4576528928, 5483790333, 8307958948, 2922357136, 7204184804, 5822687302, 5775616663, 9339608514, 2258246283, 7913922861, 5424998436, 2569675454, 2999613437, 1028481997, 4799980484, 9989507970, 6745067192, 6637131276, 132420465, 8605262021, 3753797575, 4937033346, 8499040350, 1219715423, 2578924339, 8924218631, 1715289862, 7543254491, 4876471755, 8635388298, 1667117800, 8803400324, 1041362135, 6573526219, 7131621605, 172206767, 1704660536, 651179719, 3877450754, 7073666567, 622658486, 413814158, 5276405073, 7914878641, 8326169243, 9457050726, 7391965048, 345996429, 7084029411, 4785298317, 1428552452, 194127470, 577531202, 7854364711, 3186118575, 2277625676, 9701216225, 4853417616, 35647199, 6972123626, 9524587606, 7473441264, 1426804806, 6974015925, 7669345941, 6771211396, 5139441112, 8808447783, 7774268089, 983966739, 2870814254, 1485543578, 9755550860, 7950610914, 5399844713, 6323046969, 7286202202, 2647398447, 4226223067, 2633592339, 3228913744, 3029729329, 732170985, 4737562079, 1685124860, 3585839428, 1840300688, 7103021643, 2243107793, 6639896200, 8387777238, 8861889884];



static mut KEY_INDEX: u32 = 0;

fn rc4<'a>(data: &'a [u8], key: &'a [u8], out: &'a mut Vec<u8>){

    let mut state: [u8; 256] = [0; 256];
    
    let mut idx: u8  = 0;
    let mut idx2: u8 = 0 ;

    for (i, x) in state.iter_mut().enumerate() {
        *x = i as u8
    }

    for i in 0..256 {
        idx = idx.wrapping_add(state[i]).wrapping_add(key[i % key.len()]);
        state.swap(i, idx as usize);
    }


    idx  = 0;
    //idx2 = 0;

    for x in data.iter(){
        idx  = idx.wrapping_add(1) ;
        idx2 = idx2.wrapping_add(state[idx as usize]);

        state.swap(idx as usize, idx2 as usize);

        let tmp = state[(state[idx as usize].wrapping_add(state[idx2 as usize])) as usize];

        out.push( *x ^ tmp as u8);

    }
    

}


fn handle_syscall(emu: &mut Emulator) -> Result<(), VmExit> {
    let num = emu.reg(Register::A7);

      match num {
          214 => {
              // brk()
              let req_base = emu.reg(Register::A0) as i64;
              let cur_base = emu.memory.allocate(0).unwrap();

              let increment = if req_base !=0 {
                  (req_base as i64).checked_sub(cur_base.0 as i64)
                  .ok_or(VmExit::SyscallIntegerOverflow)?
              } else {
                  0
              };

              // We don't handle negative brks yet
              assert!(increment >= 0);

              // Attempt to extend data section by increment
              if let Some(_base) = emu.memory.allocate(increment as usize) {
                  let new_base = cur_base.0 + increment as usize;
                  emu.set_reg(Register::A0, new_base as u64);
              } else {
                  emu.set_reg(Register::A0, !0);
              }

              Ok(())
          },
          63 => {
              // read
              let fd    = emu.reg(Register::A0);
              let buf   = emu.reg(Register::A1);
              let _count = emu.reg(Register::A2);

              //println!("in read, buf ={:#x}, count= {:#x}", buf, count);

              if fd == 0 {
                  let mut buffer = String::new();

                  if let Ok(n) = io::stdin().read_line(&mut buffer) {
                      emu.set_reg(Register::A0, n as u64);
                  }
                  
                  // replacing newline with null-bytes
                  let buffer = buffer.replace("\n", "\x00");
                  emu.memory.write_from(VirtAddr(buf as usize), buffer.as_bytes()).unwrap();

              } else if fd == 1010 {
                  let content = std::fs::read("readme").ok().unwrap();
                  emu.memory.write_from(VirtAddr(buf as usize), &content).unwrap();
                  emu.set_reg(Register::A0, content.len() as u64);
            
              } else if fd == 2020 {
                  let content = std::fs::read("flag").ok().unwrap();
                  emu.memory.write_from(VirtAddr(buf as usize), &content).unwrap();
                  emu.set_reg(Register::A0, content.len() as u64);
              } 
              else {
                  println!("Not handling = {}", fd);
                  emu.set_reg(Register::A0, !0);

              }
              Ok(())

          }
          64 => {
              // write
              let fd  = emu.reg(Register::A0);
              let buf = emu.reg(Register::A1);
              let len = emu.reg(Register::A2);

              if fd == 1 || fd == 2 {
                  // writes to stdout and stderr
                  let bytes = emu.memory.peek(VirtAddr(buf as usize),
                  len as usize, Perm(PERM_READ))?;
                  
                  if VERBOSE_PRINTS {
                      if let Ok(st) = core::str::from_utf8(bytes){
                          unsafe {
                              let key = KEYS[KEY_INDEX as usize];
                              //println!("{}, key : {}, KEY_INDEX : {}", st, key, KEY_INDEX);
                              let mut out = Vec::new();
                              rc4(st.as_bytes(), key.as_bytes(), &mut out);

                              std::io::stdout().write_all(&out[..]).unwrap();
                              std::io::stdout().flush().unwrap();
                              println!();
                            
                        }

                      }
                  }
                  emu.set_reg(Register::A0, len);
              } else {
                  // Unknown FD
                  emu.set_reg(Register::A0, !0);
              }
              Ok(())

          }
          80 => {
              //fstat
              let _fd          = emu.reg(Register::A0);
              let mut statbuf = emu.reg(Register::A1) as usize;

              // writing 0 to `stat` buffer
              // size is not known
              for _ in 0..0x90 {
                  emu.memory.write::<u8>(VirtAddr(statbuf), 0).unwrap();
                  statbuf += 1

              }
              
              // Setting the returned value as success
              emu.set_reg(Register::A0, 0);

              Ok(())
          },
          169 => {
              //gettimeofday
              let  timeval  = emu.reg(Register::A0);
              let _timezone = emu.reg(Register::A1);

              let tv_sec = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

              // The value returned by this will be used in srand()
              unsafe {
                  let time = TIME[KEY_INDEX as usize].wrapping_add(tv_sec);
                  emu.memory.write::<u64>(VirtAddr(timeval as usize), time).unwrap();
              }

              // Setting the returned value as success
              emu.set_reg(Register::A0, 0);
              Ok(())
          },
          1024 => {
              // Open()
              let filename = emu.reg(Register::A0) as usize;
              let _flag    = emu.reg(Register::A1);
              let _mode    = emu.reg(Register::A2);

              // Determine the length of filename
              let mut fnlen = 0;
              while emu.memory.read::<u8>(VirtAddr(filename + fnlen))? != 0 {
                  fnlen += 1;
              }

              // get the filename bytes
              let bytes = emu.memory.peek(VirtAddr(filename), fnlen, Perm(PERM_READ))?;

              if bytes == b"readme" {
                  emu.set_reg(Register::A0, 2020);
              }
              else {
                  // Unknown filename
                  println!("Unkown filename:  {:?}", core::str::from_utf8(bytes));
                  emu.set_reg(Register::A0, !0);

              }

              Ok(())
          }
          62 => {
              // lseek()
              let _fd     = emu.reg(Register::A0);
              let _offset = emu.reg(Register::A1);
              let _whence = emu.reg(Register::A2);
              
              // Not implemented

              Ok(())
          }
          57 => {
              // Close()

              //Just return success for now
              emu.set_reg(Register::A0, 0);
              Ok(())
          }
          93 => {
              // exit()
              Err(VmExit::Exit)
          }
          _ => {
              unimplemented!("syscall {} at {:x}", num, emu.reg(Register::Pc));
          }
      }
}

// Emulator is based on gamozolabs https://www.youtube.com/watch?v=iM3s8-umRO0  
fn main() {

    let mut emu = Emulator::new(64 * 1024 * 1024);

    unsafe {
        KEY_INDEX = rand::thread_rng().gen_range(0..512);
   }


    let rust_off = 3424440;

    emu.memory.load(
        "./rustmeup",&[
        Sections{
            file_offset: 0x000000000000000 + rust_off,
            virt_addr: VirtAddr(0x10000),
            file_size: 0x00000000001db50,
            mem_size: 0x000000000001db50, 
            permissions: Perm(PERM_READ | PERM_EXEC),
        },

        Sections{
            file_offset: 0x000000000001db50 + rust_off,
            virt_addr: VirtAddr(0x2eb50),
            file_size: 0x00000000000120a,
            mem_size: 0x00000000000012a0, 
            permissions: Perm(PERM_READ | PERM_WRITE),
        },
        ]);

        // Set the program entry point 
        emu.set_reg(Register::Pc, 0x100c8);

        // Set up a stack
        let stack_size: u64 = 32 * 1024; 
        let stack = emu.memory.allocate(stack_size as usize)
            .expect("Failed to allocate a stack");

        emu.set_reg(Register::Sp, stack.0 as u64 + stack_size);


        let app_name = emu.memory.allocate(4096)
            .expect("Failed to allocate");
        emu.memory.write_from(app_name, b"rustmeup\0")
            .unwrap();


        macro_rules! push{
            ($expr:expr) => {
                let sp = emu.reg(Register::Sp) - 
                    core::mem::size_of_val(&$expr) as u64;
                emu.memory.write(VirtAddr(sp as usize), $expr)
                    .expect("Push failed\n");
                emu.set_reg(Register::Sp, sp);
            }
        }

        push!(0u64);   // Auxp
        push!(0u64);   // Envp
        push!(0u64);   // Argv end
        push!(app_name.0); // Argv
        push!(1u64);   // Argc


        loop {

            let _vmexit = loop {
                let vmexit = emu.run( )
                  .expect_err("Failed to execute");

              match vmexit {
                  VmExit::Syscall => {
                      if let Err(vmexit) = handle_syscall(&mut emu){
                          break vmexit;
                      }
                      // Advance PC
                      let pc = emu.reg(Register::Pc);
                      emu.set_reg(Register::Pc, pc.wrapping_add(4));
                  }
                _ => break vmexit,
              }
          };
            // print if err otherwise program will not exit
            if _vmexit != VmExit::Exit {
                panic!(" {:#x} {:?}\n", emu.reg(Register::Pc), _vmexit);
            }
        }

}

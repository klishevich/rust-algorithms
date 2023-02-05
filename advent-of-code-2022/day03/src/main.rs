use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    part2();
}

fn part2() {
    let mut res: u32 = 0;
    let mut all_res: Vec<u8> = Vec::new();
    let ddd: Vec<&str> =  get_test_input().split("\n").collect();
    for ee in ddd.chunks(3) {
        let mut vvv: Vec<HashSet<&u8>> = Vec::new();
        for &f in ee {
            let bytes_arr = f.as_bytes();
            let b: HashSet<&u8> = HashSet::from_iter(bytes_arr);
            vvv.push(b);
        }
        'inner: for fff in &vvv[0] {
            if vvv[1].contains(fff) && vvv[2].contains(fff) {
                all_res.push(**fff);
                break 'inner;
            }
        }
    }
    println!("{:?}", all_res);
    for e in all_res {
        let e1: u32 = e.try_into().unwrap();
        if 97 <= e && e <= 122 {
            res += 1 + e1 - 97;
        }
        if 65 <= e && e <= 90 {
            res += 27 + e1 - 65;
        }
    }
    println!("{}", res);
}

fn part1() {
    let mut res: u32 = 0;
    let mut all_res: Vec<u8> = Vec::new();
    for row in get_real_input().split("\n") {
        let bytes_arr = row.as_bytes();
        let len = bytes_arr.len();
        let half_len = len / 2;
        if half_len * 2 != len {
            panic!("half_len * 2 != len");
        }
        let mut items_map: HashMap<u8, bool> = HashMap::new();
        for &e in bytes_arr.iter().take(half_len) {
            items_map.insert(e, true);
        }
        'inner: for e in bytes_arr.iter().skip(half_len) {
            if items_map.contains_key(e) {
                all_res.push(*e);
                break 'inner;
            }
        }
    }
    println!("{:?}", all_res);
    for e in all_res {
        let e1: u32 = e.try_into().unwrap();
        if 97 <= e && e <= 122 {
            res += 1 + e1 - 97;
        }
        if 65 <= e && e <= 90 {
            res += 27 + e1 - 65;
        }
    }
    println!("{}", res);
}

fn get_test_input<'life>() -> &'life str {
    &r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
}

fn get_real_input<'life>() -> &'life str {
    &r"qFdBBvtHHfvRlfvsqldvqjPpQmnQmjnjjjTRTLGRNG
ZCWhhCsJCzSJzSbzgsmPTGNNPPNGjgLTLjgn
WJZsbJMwJcszJcScwhVltFwBFBlqddvFdHDfqq
crtTsGTtqFThGQGCrsjTwdNJwpRdnJJwffRClpSf
PWVBPVHLvHHVgvZWBzmPpnfRSJJRQnSRflRPSNSl
gmzBzDgzmZQWLDLLgVmDrqGhsscrqDMGhcqtqcFr
HsbcdVrsbVbcLfPqqQsqqtPj
mMBFzZRnmFMRBDnDFBGZDGdDqLjtdQtPtgfPfttgtqgq
BZvZZdJMBFdJhSvhbhchcHll
GNRSqRfcNTpfGCcqjfzBpDQPWBzgDpQsPWzW
rrSdnVHlbMdLdBDzgtBtBmQt
rbFwwnLFLFwlMLrFwFhMVLrGNSTfZTRhfTqjGJRRZTCNcf
QWTnQCnWNNWmTnSPQwmqDbcscbpcjPjVPbrjpq
vJhzZNlNNgdzgzJdlGzHHcHDpjsHqrvbVrbvrD
RzRdRlhLgtCwCWSLnN
SFTJFTTwTVVSJBnSTdvNNfWbZCZWNZCNNhBv
srLrcHDcsjtLcLLcrLctjlcvbDNhmWCvNhZWGZZhNvhZmb
rclgtMPrrSgVTgJCng
DbrhDzcDffbzNbZvZWSSqSTNSVWv
gCPltPmCPglFnPFwtGPhGPwTCTdZZWZVRvWqdRqVVdTdvR
hLBhlmlstcffBzrpfj
wFLLmhMfwZLDwmMNRhZwRLDvJgldbJHPdQvcQHHJQPgH
bjrVrTSSJdQHcVll
CGCSsCCBpspBrqbSttpbqWmWZRmfFRZhZMNNLFqFLm
zWGjjBHGjzzTWMjhtDDWtPPlJZPJpvqQrmZTqQQpmr
RFbVLcBVLRcRVcCsCCqvpCZqmplqQJmPrlvQ
FLNRRSSRgScSVLLLNdFdwjHjnftBtGMgMjzHgzjWjj
znVSqnqbqzSbzTHqDDZmlcFcnhDMnDmn
LtjsvdvLJdjfFwRRCCMlChwCpMcclCcZ
LgvjjfjFQVgNTgWq
SJRJRFFCMSsGRMMwtZJRCVTgqgTVgTBCVpjTjmmWlB
ccvnnpnDVqTcBVTV
vPHprdHdpnzHSMsSrMRZJGws
GddGrcGNHnGvnCHddvCSWqTSWsTwTWShbHlhhb
gDPzLRVZgQfpRRFQDDVFDfzhSzsTBqqqnqbhnWTSSlST
QVFfFgRQQgLtgffZRfpFPfntjrcrjCmtCdMMmjMdJJJNtm
jjmNcpGCNmDqqsBfnZnGGGRLsZ
lrmlVWlQQtWllgtbQVrWBnZsJgsRLfZLhZBBBffL
rWMVQtrFlbFlSSMHVSdHHNHdcdDcddzppzzm
bTpjpjcVTLmphbLppJwqzqwJLqqzzzgRLJ
sdHNbrvNHrqPvZZZPRww
bNQCrCNtNsSlhffhVhpVWFCW
lpNnpMMZZDbNbnBjcrbjvScFmbGj
wqhdqVqdscrjdLsv
HQftVqWCfhwqtCCjWwfqzzVPZRJQgMlggZMMMZTNMNTnNRTN
fvvGbFtVmtTwgtMT
WcCcClzPCCcczJJScPWWZzBDmwbhBBHSghgDDMTHMDBD
nWPljWzZWnbcbRsNFjFFdFdVjFsj
NQrcLNmQGRfGLHHLZgbbnpjZJJJndbgnlv
DWtThDWtzzhltWTwjbdpvjbgqjgg
VtSPFWtBPBFSFBWCStshWBmlRfHfMRcfQLQLLlmCrCcN
pbmwqJnqSJVwwDPCjZZzrZfD
QtssBTvNdNvNtQvQGpGhdjPjDjczZDfjhgPPDcgjgr
GltptQpMGNNpRWlWFVFHJFHLWH
ZLLsDGGVhZcQQLhrLshrVFwHnWqJnWMnJJJnqfWfGn
jMlPdTlPlgCgFpngFWFnJfpw
TlTNbdSSTSTmTjPMTCdBPjBMrLDczsZcNrDhRNDRQRLLRVVz
HDLpBqDVVTvwGDDNRT
PlVWjfhsPMMmWtlFNTrhrrvCCCTNNbvw
lsglfgVJmsfMjJfSqSzdZnLgqcnLnp
pfCDJWBpfDffpJLgQJzzVzNrgNgNgNhNzmVr
ZnnGZbGTPZnsnRFdTlbrwdrNzrrmmWwmwVwttH
GbPGRvTnZljWnpqSMMCjqJQSCf
ZgnFgwggznFrfrwfHhNMMr
pctLCLRhPHBLMLWfBL
JJcdJcQCCJmQJppmlgndnFslsVnsvghZ
WpMgTppWGSWWJmJDpJcJJhqm
zZzjZNHvNjPvNsbZLbRLzsPcqhVJSVttdwhwmdRhtdJRVd
sLbvvCZCPSSSbbPfNlQQTQGBllCTnMnWQn
fwbwswddwSbBfDBggMBPDPhHcPWDmhHhmWnWPC
FQFlzLCzQTlrTTzvltFqFrmhPHjnhhnnchcJWcRRmRRq
lpLzlFZzCltrTNlTztQLZfSMGBNdSBVwbBNVSMSbVs
FMmgbTFdgLSgFQdjrRPrQBPDdj
ZqqWRvsfGrrPvvPC
wZzwnqccRwRNNpRSMztSMMFbgzLTFS
qTwBPfTfqQDMDrssHdvtRHccHMjR
gWSZGWzGFhnFFgnhNsRHtRdsVjZcRjHs
jgplhpJJFgnDrrwfqprwDP
CWhMSRfWhVVnRSZnVVdsLQqQMzGqLBvGMQqczv
PHbpNwrjJplttvcclLlQzzDszc
NrJbJrFNNJNPrmwrtbjtNmCfSWfWhSZZfSWCsfShfFVR
VLhRPLGLRPRSStRRLwfGqfmDwbmqbqqDlD
rBSFvppnzTbwDwlDcFWm
MJrnJTMvMsrTsPtshRNPZdSLhL
BZBrRCrnCQBBnZfGqhGGMMRcthMhMG
TLjsCdDCPTvNssjdsPsDgsgqGcPHczchtHczWzPWzzlWhG
gsTpsdNbvNNjNSpsNDTsmCnSVQFmSFwZnQBnmnQQ
llbsNsWrmbrGbWCNtBjcCFBzQFZBCFjF
LdSpwgdqSgzwJdRdLMRHLjQQjHjFHctjHBDTZj
gSppgpSJMhpzwrhblfbhhlWlnW
DwhTvvsJZWsBnDzPpBLbFp
GHtNGRGNdzbMBBtmBt
NljlCSVSHdjGSQRGlCQSCswqfWzhZfTcfzcJvshJ
lmsGNFsDGqCbFQBbffjjwpzptw
hRQdvdrvrvSngWnvnHrTMfzfzRtftzwVTwwpzB
HnSSWrvLJvWJGFDsmFLPDFcQ
bwwpGphpLghpTvpWphvJlFLJqqltjSjVlSStSR
cmszZdDdBZzcNcDCDcNsmNMcqVjMJStFRJltVPVrlVPjVJll
HcdmcCzzzQcHNcsCdpnGnhwGgnRggHvbbR
CfMBbwBGbMbDCFrDvhFFDT
mjzRjjRdSmjPnzFZgnnrTT
cmSsVcHjLHTwMfLBpBpBwM
whqqfZzgHvhSzzVNVDbpDbmbVbNpJD
GcQFntGTCCcCTMCTGBlJsJsDDWpRbWBsJpNS
FnPcrGFFdddMnCnTqgqgqPHfLjLqgSzz
zMSzzjssFdGnszRtNftqqwFHbbZw
RRPLVrgrwHqBqgwt
rPWmLCTCQlCQQmmrWLrQShJshhzdhhJjcSjlzRds
lvgvCDfPqLHppqpCCDJncbntttbBtBBVHjwtrB
TdddszSQsWcngjzVbcVZ
hRWsTRTGQhNRGhRTFSWmlpgfqlvLmplPqvvGgv
LbWFLQdWWPwWSjSHPHRfppHHDRpggR
zmqqNNzlzmnzzNCmVCmtBzpfGsfpBgDgspprcfcfsrRB
qNNVNJtNmmmNzznVJzvCTDZWhvZZjZFbWQQhFhbZSw
DjdHqJVVhHVZjhDHPWtDtZLFBRBFmSRTFSbwmRRTffTTJf
NNznnGlgMQsnQzNclzpfSRSMRmfPMmFRwBwB
vzrcGcNcPPvHvHPt
wLCcmZwWTNtZNdMSMGSCnJGGMB
RFbHsPhVvFPRjlshhrnQnGjQGSdSqJfqnQBM
HhzVlFHhPwzScmSTgL
TNlBhDNvNBFpJgpPPpDQ
jjfCdCZZqsCZsbdqPgFGGMRzSFMqQMRS
jnWPtWssCtWcmZbbtstvnrrvhVBhTNNhBHlBlL
DZwNWPDzPVWbJngrQjrNnrQcMg
GRRfttLBhhvTvmLmFcFcgFFSnjWrnsrG
TLthBWtTRLHqhlLLfmhBqVPDJVdPwzJCPPZHwdDdVd
GGVhrVSMQwQqfVssVvnWFgvgWn
jtlcRBBtQRmpWsjzFCvzWnvF
QPcRbpppDmNDtPPblZMfhZdDwdMrqSSGrq
ZRrdtBdQvQsWnnfWFZsF
bJLcMzNDLbMgwfnGMWFv
lpvhmzNDmDmlNbzbmrVVPrHRCPHQBVCP
rZllQrsRWrlQswccMVbGbVbTdcQQ
NtJCntLSHCjznfLTcGGGqWMdWM
jCtzzSFthhSSSjPJrFDlvWrlDZRpwpRZ
mQmbLjbrLQjLmTtwwWBTTvWjtt
BHSqdHclHHNFlppNqWPwfwDvTfDPPtCw
ddSGMGHcdcMhMZnBbmbZmgGJJg
lvvBzvDnlzjfPnfjnQPlldRbVbRqbqqCgsqqVpQQgVqc
NNFtGNMtTNFmJNGNZtZMwVRTTcsCpVTbbgCbgRhscp
FGNGZMtNLWmmJWGFWJGLSNtPrPnBfDzzvjnDBzpnvDBLnv
fwvQRFQvQqwpwNJrwN
BstDnBjhjBhnshSptpJzWqNppbfr
CsDjCdZcBCDcjnfDHfhnfggZMGlgQVmgMTRmgVGMMl
MwlBVqVlsgnmzwJsvjhWZhGPvjvRRWzG
QNQpQpftHdHHCHGfSpCrQNdSrDRDhchhjvjcPrRRWrPvhZjv
LtLSCTSGfHGdGwswnqsggssTqV
qDDCHjzjznTvWshZQWfnZZ
PFFmmNMMtNMVFtczcFPJNrLhZwQZQsSvvSvWvGQQJQssss
tFzrrPPNNFlzVrpRTpblRDqjTpDC
DWDrrBdpmdpBrCgDthdtfcHsqJsCqscqwfsjzHcq
TNLNFNSTQNQTSnlMcczVJjVzsqLDDfJJ
TFPZQRvvlMSPPtRWDtmDRWrBGr
LWGVZdrvWdpLGWRsjPMsHmdHdHldlj
zJzznChzzzCSfTgMhCPDmlDCbmlsmjDDQj
nSTTJhJtnShNtzwhgNrGRRWZZRvMWMtVrqGp
PbPmtNmBbPlqBvqlDJBT
LpGVDzVpVZqqSTvq
pMnWGLRLRppnGpGndrGPtgDCjMPmbPgCQmPPNN
sqcZcbZZpcZspcCCRMmznWGWdLWhwDRGTTWggT
NjFSJgVHrvfVtrGzWdSznDwLSTLn
jFrBNVVjBFNvHrFHBlBFFpMslPgPcpMPmcQPPZCgpP
frddqsThtsTfTbPcvhsrbsRLpRBNRpmDpGmRGcRNLpGp
QWJHCJwWzlHZQZHQCJJRzRqnLDGRGpnGBRnNDN
CVwHCClJjQgWCZVZQgMwSdthjrqvrSPPhdbqtPhs
TvdphBBhhhCgdLNNJJJLWz
fVcsqRVrPcnJWgDnJN
JlqsRJtssZwqwVtPwltRPsHHbFTwTFbpjHhQjTQbvpTF
cQSnPDDQJGNzwnNpZb
RHDrssVRDHRgsRFHRlrVwzzpNGZlfZdppZdwGNZb
sHCHtDgtCjVVLFChqPMhBCMcSTqB
hdbQbqcCCQcqFbCbVdcWCQQlRMBtGlRHBtBMpHhpHThZMR
LLsSLLfgJPrgPnssnmlZtlZpHGHVGfZVtZpl
PvmvgmvvnzmrSsSLJDqDNzqFDQdDwzWWbV
HNNjnLbpLGHvWJDhdWWPpWDW
lVcSNgcSVclhRlPZPRCDCR
cqmSQrwwrrVSrtQFqVNmFwjQnvjHzBbLLGjfjzHTzvnH
QmvWVppPHQQvbbvmSHSpPzfzwnWMTZFFzwFMCzLnwT
jGBljlNNjgDtGDrNjjtjqqDRnMzRLnFzCFnMfRfMzCnttF
jqNrrGdJcdgLjqDqBrDQbbmhdQQmmpPbphmbVv
ZHQCggVHHRDWvbfjGptVtLvL
nnFwnwrDDMShnhFrFLLLpjvPlPGGtLGb
dcNSMhrrTDCBCsWgCTQW
HqDDLGtDdCnhfDnwnV
PmlJsJTPlbdBTzTnzhnnCCWWzV
lSPjMScggsScgjSMMbqHLFGrRLGHRZZtdrcG
ZVVtNNppdZSdLtCPqnHhqJJFtb
zgwwQBfwmGgSrDfgrrGBggzHCnbJbqbCJFnqhHBhnHHCqJ
rvrzfmlRrgDgmrzfggvwzvdjjcccLjMjVcVcsVLjVSZR
dppcLRHpphchhNhSddjzHzWQWQLtrMsrWQCWCsMZssCZ
JGfBfJJfBqvGVlVbDBwDBDBfZnrQsMQtssMttssDsQMWZncn
qPVwlgPBmjpPhcmS
zGPnzBgPzPnPlHZlDDHnZBNCvrtcjcjmMcFzNcNFmFdc
qQpfsLTTSspqTfJdmdCtMjdtjvJcmr
bfQqqSrswLLrfpLTqprfTnDVDVBBbgHPDHnhDPgDbV
JssTnsdFztZLdNJnNtTsLNZGqlbGFBqrGMHqHBcFBqMFMH
CCgSfgPSvSfhpShSRppCdfrlqGHGGcHmclmqbbqbqlPc
wvVSVjQSSQhRVvfQChvZZsdtJstjLNLZDJnLss
CmfNNNZNqDrnDjMhZM
gdczzGtdFcddtWQgGGMnVhnjJwnrJFDPTwMP
dlvcdzdHtzQSLRSfmhLSqv
ZpFFLcHFZZRRmJVZgD
PzhrtQntzcrjCRJtbtRgBsBRVR
zdzWfCzhQzlhWfWhlvpFNlpSqcMSHHMv
NrrMgMhNQhNjQrtqtPtwVtZpggPw
TfRLndnLFCRFTFbbRDHwpVqqBBwsHwZsfH
TJFRdLlRThrlcvZcvQ
scrwRVjbQvQBzsBC
gMfVqNnVmnCBQDTvdn
SMqhWqVlmWSmqMVRSJjjpcFrcLpJrR
HtSQHQntHsHMrtHnGfHQVVzLvSBSVvVVSFNJzzVN
cmPRmpqlpPmcgTlTpjJNjjVDvDRFNVVBFD
hlmCpmqmpgqpZTlcdQHFQfbHHZttwMQwtr
VpWgbgfwCjbftwVPPpGQFQhzTBQTBGPzqFTS
dbRbDcRrsnsRrLZmLRDZldDZqTNTGqqFFzGGhSTNTFTzNmNT
MlLdHlDDHrHclMMrCwgHCCwWwCbCCjjg
GGNLhfDMVcVrcGsT
jSJQFjHbwPFSvQSHwZFvHSHrqCCrrTsqBwNBrcBNsVTsqq
QjZSjZJZPvNRZJQPnSZbJZRWLfnmgDlmhdhWgWLdMdfmhM
CgGnzPNggCJtNTgTZTPZzZZvvcDcDDdqDFcJssJDHDqvHq
jhhrrLVlmLRRnRflfVbFHHHqdVsDqcvbHVDb
jWfWwrlmRRnQmPzZNGZPBNCQTB
NzDDhwNmhvtrGmNCvWRVbcRRVTcHHcVFTbwV
LgsPlLsQgQdJsLdldtpgFFTMbnFqTcMbHqFcPncq
dgsJsLLLggljrhtGNNtSjvGm
ptzSrZtzhsmmtPrhLFRFnjnnLMsnfLRL
HvwVDHwWWgGDGdHgqVDWDMnRnTjFNTNjfLJvRRRRRR
DwDgWgQbDDDHwBBBWdwQGVHhlhlZZSSmztfcppSBhzZcZp
CWmWRzlMJqWDWqCJbqDlCBBVLMQHVMGrfMVtQZrsLL
SnhPdFFPNZsBBdHtVQ
SSPcFFgnwnSpwvcSjwzCqRzTmJbpJCRBmbbD
wQbqGWWSqwrbGWWWGjbNMJPfgfnnDmPnPNLfjN
tJFztRZCvVRCztZFZRVgmMhmgNLfRfnmDPNPhm
BFCVZzpVFlHCdbQqcTcGlJbbSG
tttfLPZZQZTlZPHHPWgMVvBnjmvjnjgGBQ
FzcNDDDrNzprrrshprhFJtVGVnjtjGvnhvVnnjnjGM
RDqJNszDPfdqPtlT
QCJdMjCQbdBjSbTHDsbWDDwHTP
zlvlmqzqGfgdNzLldrHwwPGpWDrPGZWprr
gfVfRczVqcRzmdcSQMjQSQCSjQCQ
RhhCGhRBShjjRfpwppFTfFHZHZZD
qzdqzlnPPctPdmtPdTZbwQvvwqvHHvZpwZ
nVTVTcsWmWSRhhRVrGVB
GmshRMnzqRGsPNwMwcrrpcVV
CDCbFCLvCgfDSFLslgDpwpLtTwwcPtNNtTTprt
JvSFbSbbFllJlgDvlJbgdRhRdqzBGnzshZnRRRnHBZ
ShJhtcsvvvQbnnsccVTLVTppWqddpVnLWp
NdzPrPZgPMNNrmzpTzpTCjWfzCpzVL
dZgHmZRPPZRlZmrPDtDRccvbtQQbJbRS
wqjLjwhznhBLqLWGfvSlvcmlrJsqrtJTJJ
PwbpFPQDRCDrDJTrTmvs
gbZVFbZHgwHbCdpCRMnffNLhWnnzMdzLLW
RVVGSNTTRlNqHblBNB
JfwJMvLLZwLsMJwWMJfwLHBqzFlvzpBQcqzblFbBqblq
wMCZJsgJCCCnsMHrgLLjSPSVTgShtRjTPhRRmt
lmQSSWdMHHLWgWqD
ZZtVGGGJrJvGVCwfgHNLccmNFFcqtc
vrCGPhvrTPdRBnsRTmmp
dDMDDjzCQjwCCcDgjSLLLsLNlmpplN
FqrHFTFRLCLVFBmS
JhHJhHRThrfPZPvhnTZZbWdwdwDDWtDzJDtbMtCW
ghwDzJRDwHmPthncSPncLLsPcvnv
MWCrNTCHrMVjQQMQcSdnpTLnFFdTcnTc
qbWMfWfNrWVQWfbjVBbqMfwDtqzmhmRRzGhtHhHhRwZh
fmSmnjTjrlzGlTzJdH
BrhRRQMrgQvgFFhQQbwpFvGGdZqZJqpJqHVpJGJLHdLJ
ggbDwQMsvsMQrFMFcWSPSCPmSsPfnnmP
cmNVbMrrrjcHDRcvfW
wQGdFfSThFsLhhHWvDCWDCJRCCjd
LtpStGhqrrpnfnpp
bvcccTqbgvpGndJtgdsgNd
wDQwQhtQhQRmSmjsJndJdBBJBJnlLS
hwhmRrzFVjtwzDmrVFrvPCcCMVPPvfqpCCTVVb
jRrRNPNRWjPRWPRQNjQjThTCzBBzDCFBGzgDFGGQ
dnppLwmwCnvtlqltvtnTGBThGhdFZhgzDzGccD
MvnqpLlMqCqCHMjWPPHMSHSs
NNpNNvpvBdtTrMFFMhSSwzjzchzwhzwL
VVndHqflQZZZgHSLLhjzRSmZRhcR
glGgnqbQlngnWCGJpJprtrtFrdPPGs
WqwRjzGtRzZZRRGjWBJzjwmfMTHGGssTTDsrLmmmQLMD
SNdvSdFlSNNhSPFFcPFclbQQslHmfHTDsTQMLgDTmHQQ
CNcCFvpdnWpjWwJf
PVPnVHcnRncGZqbVzHVPnnLbSMjwrzWMjSwDtWwWtwWhwDWz
pTfsQCshCllpglWWSjBMSQSrMrjM
hvpvppggCpJTvTmshgfsmZRRHqbcLPHZmPLRnmPZ
LQbhVZZmZhZjBdbGmgHqnHTmvqgnnWHr
SzCfDFFNRfsSFFMFfvprvpWzqzgqTwHTvp
CDNDFJgMDSQhjVdPJLQG
plpdLdpjjrrHJJjLrrHLFdbzzCcvzgFgcwggzPMFvvcMhM
GRtSBQNsQlMPRzRlzw
ZSTtsmBlmjLLpnpH
hglGNVSdNSghzSgCBhDFLBMBtFMMFtHtbtLL
frQZccRcqGFmFHrJ
nvfGZwvTwGTfQwvfTwfgnCSlpdnzgzslppCsCV
snTSPbQnTTnQgbmsTJsLfZwjffhpLnGRjpGfjL
dcNWcNHHlNtWHHlCtltWNFNMLZwjpGfpmrZfrFprrRGpwZfp
HmdNWCmDMVvQPDgqJs
GGFtSngQLfnSnQffgPnRgFRGRwmRJvwbBbJDwjvTbjrwhJvJ
WHClslcNNWcqNWlCZdcHsVrThBwBjbhDTDBhrvDZJTwm
NWVqqcHHNpsNcNVdVlhCMlHQQMQQzLfzQPttFGPMLSLgtF"
}
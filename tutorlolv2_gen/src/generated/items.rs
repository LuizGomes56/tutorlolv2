use super::*;
pub static ITEM_GENERATOR: [Range<usize>; ItemId::VARIANTS] = [0..0,3957541..3958405,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,3993682..3994555,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,4057071..4057821,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,4079639..4080505,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,4115636..4116506,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,4140215..4141078,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,4160794..4161659,0..0,0..0,0..0,0..0,0..0,0..0,4175968..4176840,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,4194718..4195703,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,4219423..4220283,0..0,4223565..4224439,0..0,0..0,0..0,4231762..4232635,0..0,0..0,0..0,4240181..4241050,0..0,0..0,0..0,0..0,0..0,0..0,0..0,4258214..4259088,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,4346592..4347461,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,4380721..4381474,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,4431780..4432649,0..0,0..0,0..0,0..0,0..0,4443373..4444125,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,4481714..4482696,0..0,4486904..4487884,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,4523153..4524018,4527599..4528586,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,4552482..4553345,0..0,0..0,0..0,4564085..4564949,4567472..4568452,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,0..0,];pub static ITEM_CLOSURES: [[[Range<usize>; 2]; 2]; ItemId::VARIANTS] = [[[3952072..3952367, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[3973309..3973533, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 3980785..3981079]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[3987567..3987789, 0..0], [0..0, 0..0]],[[3989920..3990169, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[3996739..3997125, 0..0], [0..0, 3996352..3996739]],[[4000203..4000504, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4011799..4012198, 0..0], [0..0, 4012198..4012598]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4024575..4024797, 0..0], [0..0, 4024352..4024575]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4038573..4038801, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 4048698..4048921]],[[0..0, 0..0], [0..0, 0..0]],[[4052990..4053227, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 4077535..4077756]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 4095511..4095732]],[[0..0, 0..0], [0..0, 0..0]],[[4099575..4099797, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4104990..4105479, 0..0], [0..0, 4105479..4105969]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 4116506..4116729]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4137355..4137674, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 4158694..4158914]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4165533..4165860, 0..0], [0..0, 4165205..4165533]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 4190909..4191352]],[[4195703..4196140, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4207232..4207455, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4217033..4217315, 0..0], [0..0, 4217315..4217598]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4224439..4224662, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 4234538..4234766]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 4243048..4243277]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 4247253..4247581]],[[0..0, 0..0], [0..0, 0..0]],[[4252367..4252589, 0..0], [0..0, 4252144..4252367]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 4264442..4264745]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4281322..4281613, 0..0], [0..0, 4281613..4281905]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4302112..4302411, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4344372..4344595, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 4355041..4355364]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4370466..4370687, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 4385377..4385669]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4396091..4396383, 0..0], [0..0, 4395798..4396091]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 4400693..4400984]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 4417730..4418024]],[[0..0, 0..0], [0..0, 4420682..4421106]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 4427790..4428012]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4436180..4436417, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4449407..4449704, 0..0], [0..0, 4449704..4450002]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4456212..4456443, 0..0], [0..0, 4455980..4456212]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4467021..4467247, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 4476444..4476734]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4484451..4484705, 0..0], [0..0, 4484705..4484960]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4520640..4520927, 0..0], [0..0, 4520352..4520640]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4530474..4530866, 0..0], [0..0, 4530866..4531259]],[[4534626..4534919, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4541849..4542143, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 4550396..4550615]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 4555499..4555790]],[[0..0, 0..0], [0..0, 0..0]],[[4561407..4561722, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 4565235..4565522]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4586338..4586633, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4592077..4592301, 0..0], [0..0, 0..0]],[[4594774..4595167, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 4603804..4604026]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[4613417..4613635, 0..0], [0..0, 4613198..4613417]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],[[0..0, 0..0], [0..0, 0..0]],];pub static ITEM_FORMULAS: [Range<usize>; ItemId::VARIANTS] = [3952367..3954604,3955185..3957541,3958405..3960141,3960141..3961775,3961775..3963455,3963455..3964973,3964973..3966409,3966409..3967873,3967873..3969772,3970415..3973084,3973533..3975319,3975319..3976994,3976994..3978875,3978875..3980492,3981079..3983428,3983428..3985309,3985309..3987344,3987789..3989670,3990169..3991853,3991853..3993682,3994555..3996352,3997125..3999901,4000504..4003096,4003096..4004631,4004631..4006495,4006495..4008414,4008414..4010042,4010042..4011799,4012598..4015378,4015378..4017324,4017324..4019100,4019100..4020709,4020709..4022387,4022387..4024352,4024797..4026519,4026519..4027961,4027961..4029423,4029423..4031169,4031169..4032938,4032938..4034916,4034916..4036528,4036528..4038344,4038801..4040809,4040809..4042443,4043110..4045419,4045419..4047034,4047034..4048476,4048921..4051059,4051059..4052752,4053227..4055306,4055306..4057071,4057821..4059744,4059744..4061504,4061967..4063533,4063533..4065068,4065068..4066675,4066675..4068337,4068337..4070117,4070117..4072038,4072038..4073921,4073921..4075805,4075805..4077315,4077756..4079639,4080505..4081955,4081955..4083628,4084075..4086117,4086117..4088171,4088171..4089863,4089863..4091651,4091651..4093472,4093472..4095291,4095732..4097647,4097647..4099352,4099797..4101606,4101606..4103039,4103039..4104990,4105969..4109445,4109908..4111823,4111823..4113878,4113878..4115636,4116951..4118940,4118940..4120729,4120729..4122250,4122250..4123765,4123765..4125279,4125279..4126794,4126794..4128317,4128317..4129834,4129834..4131798,4131798..4133688,4133688..4135141,4135141..4137035,4137674..4140215,4141537..4143286,4143286..4145200,4145200..4146690,4146690..4148316,4148316..4149814,4149814..4151436,4151436..4153337,4153337..4155086,4155086..4156969,4156969..4158475,4158914..4160794,4161659..4163414,4163414..4165205,4165860..4168371,4168826..4170382,4170382..4172261,4172261..4174042,4174042..4175968,4176840..4178629,4178629..4180583,4180583..4182144,4182144..4183762,4183762..4185618,4185618..4187242,4187242..4189003,4189003..4190467,4191352..4194718,4196578..4199178,4199178..4201015,4201015..4202837,4203288..4205187,4205187..4207008,4207455..4209261,4209261..4210959,4210959..4212754,4213211..4215226,4215226..4217033,4217598..4219423,4220283..4221786,4221786..4223565,4224886..4226733,4226733..4228244,4228244..4229996,4229996..4231762,4232635..4234311,4234766..4236661,4236661..4238450,4238450..4240181,4241050..4242820,4243277..4245025,4245025..4246926,4247581..4250094,4250094..4252144,4252589..4254462,4254462..4256335,4256335..4258214,4259088..4260978,4260978..4262658,4262658..4264442,4265047..4267310,4267310..4269202,4269202..4271002,4271002..4272814,4272814..4274714,4274714..4276993,4276993..4278433,4278433..4279873,4279873..4281322,4281905..4284140,4284140..4285870,4285870..4287646,4287646..4289686,4289686..4291574,4291574..4293322,4293322..4294792,4294792..4296259,4296259..4297717,4297717..4299187,4299187..4300654,4300654..4302112,4302711..4306486,4307453..4310764,4310764..4312568,4312568..4314498,4314498..4316571,4316571..4318190,4318190..4320112,4320112..4321967,4321967..4323394,4323394..4325272,4325272..4327153,4327153..4329028,4329028..4330934,4330934..4332626,4332626..4334533,4334533..4336435,4336435..4338501,4338501..4340284,4340284..4342086,4342086..4344148,4344595..4346592,4347910..4349913,4349913..4351413,4351413..4352843,4352843..4354719,4355364..4357874,4357874..4359750,4359750..4361399,4361399..4363029,4363880..4366869,4366869..4368606,4368606..4370244,4370687..4372414,4372414..4374072,4374072..4375546,4375546..4376985,4376985..4378777,4378777..4380721,4381474..4383193,4383193..4385086,4385669..4387521,4387521..4389132,4389583..4391469,4391469..4392892,4392892..4394336,4394336..4395798,4396383..4398615,4398615..4400403,4400984..4403269,4403269..4405212,4405212..4407011,4407011..4408648,4408648..4410322,4410322..4412265,4412265..4413772,4413772..4415539,4415539..4417437,4418024..4420259,4421106..4424067,4424067..4425844,4425844..4427569,4428012..4429734,4429734..4431780,4432649..4434173,4434173..4435942,4436417..4438067,4438067..4439711,4439711..4441493,4441493..4443373,4444125..4445910,4445910..4447787,4447787..4449407,4450002..4452245,4452245..4454023,4454023..4455980,4456443..4458345,4458790..4460719,4460719..4462349,4462349..4463899,4463899..4465384,4465384..4466794,4467247..4468988,4468988..4470738,4470738..4472635,4472635..4474411,4474411..4476155,4476734..4479008,4479595..4481714,4482696..4484451,4484960..4486904,4487884..4489570,4489570..4491643,4491643..4493299,4493299..4495262,4495262..4497055,4497055..4498833,4498833..4500694,4500694..4502558,4502558..4504418,4504418..4506469,4506469..4508555,4508555..4509994,4509994..4511444,4511444..4513489,4513489..4514966,4514966..4516700,4516700..4518467,4518467..4520352,4520927..4523153,4524807..4527599,4528586..4530474,4531259..4534332,4534919..4536775,4536775..4538310,4538310..4539869,4539869..4541849,4542438..4544464,4544464..4548541,4548541..4550178,4550615..4552482,4553345..4555209,4555790..4558019,4558019..4561091,4561722..4564085,4565522..4567472,4568452..4570222,4570222..4571789,4571789..4573228,4573809..4576195,4576195..4577923,4577923..4579594,4579594..4581077,4581077..4582754,4582754..4584180,4584180..4586042,4586633..4588868,4588868..4590619,4590619..4592077,4592526..4594380,4595167..4598397,4598397..4600170,4600170..4602078,4602078..4603583,4604026..4605748,4605748..4607496,4607496..4609558,4609558..4611304,4611304..4613198,4613635..4615615,4615615..4617412,4617412..4619249,4619249..4621199,4621199..4623104,4623104..4624630,4624630..4626536,4626536..4628511,4628511..4630343,4630343..4632396,4632396..4634316,4634316..4636101,];


pub static ABYSSAL_MASK: Item = Item {
    name: "Abyssal Mask",
    tier: 3,
    price: 2650,
    stats: &[
        (StatName::AbilityHaste, 15),
        (StatName::Health, 350),
        (StatName::MagicResist, 45),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::AbyssalMask,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [abyssal_mask_ranged_min, zero],
    melee: [abyssal_mask_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 8020,
    identifiers: [
        [&[SteelcapsEffect] as &[_], &[]],
        [&[SteelcapsEffect] as &[_], &[]],
    ],
};

pub const fn abyssal_mask_melee_min(ctx: &Ctx) -> f32 {
    0.12 * ctx.steelcaps_effect
}





pub const fn abyssal_mask_ranged_min(ctx: &Ctx) -> f32 {
    0.12 * ctx.steelcaps_effect
}







pub static ACTUALIZER: Item = Item {
    name: "Actualizer",
    tier: 3,
    price: 2800,
    stats: &[
        (StatName::AbilityHaste, 10),
        (StatName::AbilityPower, 90),
        (StatName::Mana, 300),
    ],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Actualizer,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [actualizer_ranged_min, zero],
    melee: [actualizer_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 2522,
    identifiers: [[&[BonusMana] as &[_], &[]], [&[BonusMana] as &[_], &[]]],
};


pub const fn actualizer_melee_min(ctx: &Ctx) -> f32 {
    0.00005 * ctx.bonus_mana
}





pub const fn actualizer_ranged_min(ctx: &Ctx) -> f32 {
    0.00005 * ctx.bonus_mana
}







pub static AETHER_WISP: Item = Item {
    name: "Aether Wisp",
    tier: 2,
    price: 900,
    stats: &[(StatName::AbilityPower, 30), (StatName::MoveSpeed, 4)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::AetherWisp,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3113,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static AMPLIFYING_TOME: Item = Item {
    name: "Amplifying Tome",
    tier: 1,
    price: 400,
    stats: &[(StatName::AbilityPower, 20)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::AmplifyingTome,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1052,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static ANATHEMAS_CHAINS: Item = Item {
    name: "Anathema's Chains",
    tier: 3,
    price: 2500,
    stats: &[(StatName::AbilityHaste, 20), (StatName::Health, 650)],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::AnathemasChains,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 228001,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static ANTI_TOWER_SOCKS: Item = Item {
    name: "Anti-Tower Socks",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::AntiTowerSocks,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1508,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static ANVIL_VOUCHER: Item = Item {
    name: "Anvil Voucher",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::AnvilVoucher,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 9999,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static ARCANE_SWEEPER_TRINKET: Item = Item {
    name: "Arcane Sweeper (Trinket)",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::ArcaneSweeperTrinket,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3348,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static ARCHANGELS_STAFF: Item = Item {
    name: "Archangel's Staff",
    tier: 3,
    price: 2900,
    stats: &[
        (StatName::AbilityHaste, 25),
        (StatName::AbilityPower, 70),
        (StatName::Mana, 600),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ArchangelsStaff,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3003,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static ARDENT_CENSER: Item = Item {
    name: "Ardent Censer",
    tier: 3,
    price: 2200,
    stats: &[
        (StatName::AbilityPower, 45),
        (StatName::HealAndShieldPower, 10),
        (StatName::BaseManaRegen, 125),
        (StatName::MoveSpeed, 4),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ArdentCenser,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [ardent_censer_ranged_min, zero],
    melee: [ardent_censer_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3504,
    identifiers: [[&[AttackSpeed] as &[_], &[]], [&[AttackSpeed] as &[_], &[]]],
};

pub const fn ardent_censer_melee_min(ctx: &Ctx) -> f32 {
    20f32 + 0.25 * ctx.attack_speed
}





pub const fn ardent_censer_ranged_min(ctx: &Ctx) -> f32 {
    20f32 + 0.25 * ctx.attack_speed
}







pub static ARMORED_ADVANCE: Item = Item {
    name: "Armored Advance",
    tier: 3,
    price: 1200,
    stats: &[(StatName::Armor, 35), (StatName::MoveSpeedPercent, 45)],
    maps: &[SummonersRift],
    metadata: TypeMetadata {
        kind: ItemId::ArmoredAdvance,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3174,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn armored_advance_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn armored_advance_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static ATMAS_RECKONING: Item = Item {
    name: "Atma's Reckoning",
    tier: 3,
    price: 2500,
    stats: &[(StatName::CritChance, 20), (StatName::Health, 700)],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::AtmasReckoning,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 223039,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static AXIOM_ARC: Item = Item {
    name: "Axiom Arc",
    tier: 3,
    price: 2750,
    stats: &[
        (StatName::AttackDamage, 55),
        (StatName::AbilityHaste, 20),
        (StatName::Lethality, 18),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::AxiomArc,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 6696,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static BF_SWORD: Item = Item {
    name: "B. F. Sword",
    tier: 1,
    price: 1300,
    stats: &[(StatName::AttackDamage, 40)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::BFSword,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1038,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static BAMIS_CINDER: Item = Item {
    name: "Bami's Cinder",
    tier: 2,
    price: 900,
    stats: &[(StatName::AbilityHaste, 5), (StatName::Health, 150)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::BamisCinder,
        damage_type: True,
        attributes: Undefined,
    },
    ranged: [bamis_cinder_ranged_min, zero],
    melee: [bamis_cinder_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 6660,
    identifiers: [
        [&[SteelcapsEffect] as &[_], &[]],
        [&[SteelcapsEffect] as &[_], &[]],
    ],
};

pub const fn bamis_cinder_melee_min(ctx: &Ctx) -> f32 {
    15f32 + ctx.steelcaps_effect
}





pub const fn bamis_cinder_ranged_min(ctx: &Ctx) -> f32 {
    15f32 + ctx.steelcaps_effect
}







pub static BANDLEGLASS_MIRROR: Item = Item {
    name: "Bandleglass Mirror",
    tier: 3,
    price: 900,
    stats: &[
        (StatName::AbilityHaste, 10),
        (StatName::AbilityPower, 20),
        (StatName::BaseManaRegen, 100),
    ],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::BandleglassMirror,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 4642,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static BANDLEPIPES: Item = Item {
    name: "Bandlepipes",
    tier: 3,
    price: 2300,
    stats: &[
        (StatName::AbilityHaste, 15),
        (StatName::Armor, 20),
        (StatName::Health, 200),
        (StatName::MagicResist, 20),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Bandlepipes,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2524,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static BANSHEES_VEIL: Item = Item {
    name: "Banshee's Veil",
    tier: 3,
    price: 3000,
    stats: &[(StatName::AbilityPower, 105), (StatName::MagicResist, 40)],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::BansheesVeil,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3102,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn banshees_veil_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn banshees_veil_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static BASE_TURRET_REINFORCED_ARMOR_TURRET_ITEM: Item = Item {
    name: "Base Turret Reinforced Armor (Turret Item)",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::BaseTurretReinforcedArmorTurretItem,
        damage_type: True,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 1506,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn base_turret_reinforced_armor_turret_item_melee_min(
    _: &Ctx,
) -> f32 {
    0f32
}





pub const fn base_turret_reinforced_armor_turret_item_ranged_min(
    _: &Ctx,
) -> f32 {
    0f32
}







pub static BASTIONBREAKER: Item = Item {
    name: "Bastionbreaker",
    tier: 3,
    price: 3200,
    stats: &[
        (StatName::AttackDamage, 55),
        (StatName::AbilityHaste, 15),
        (StatName::Lethality, 22),
    ],
    maps: &[SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Bastionbreaker,
        damage_type: True,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2520,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static BERSERKERS_GREAVES: Item = Item {
    name: "Berserker's Greaves",
    tier: 2,
    price: 1100,
    stats: &[
        (StatName::AttackSpeed, 25),
        (StatName::MoveSpeedPercent, 45),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::BerserkersGreaves,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3006,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static BLACK_CLEAVER: Item = Item {
    name: "Black Cleaver",
    tier: 3,
    price: 3000,
    stats: &[
        (StatName::AttackDamage, 40),
        (StatName::AbilityHaste, 20),
        (StatName::Health, 400),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::BlackCleaver,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [black_cleaver_ranged_min, zero],
    melee: [black_cleaver_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3071,
    identifiers: [
        [
            &[Armor, PhysicalMultiplier, SteelcapsEffect] as &[_],
            &[PhysicalMultiplier],
        ],
        [
            &[Armor, PhysicalMultiplier, SteelcapsEffect] as &[_],
            &[PhysicalMultiplier],
        ],
    ],
};

pub const fn black_cleaver_melee_min(ctx: &Ctx) -> f32 {
    0.06 * ctx.armor + 0.3 * ctx.steelcaps_effect
}





pub const fn black_cleaver_ranged_min(ctx: &Ctx) -> f32 {
    0.06 * ctx.armor + 0.3 * ctx.steelcaps_effect
}







pub static BLACK_HOLE_GAUNTLET: Item = Item {
    name: "Black Hole Gauntlet",
    tier: 3,
    price: 0,
    stats: &[(StatName::AbilityHaste, 25), (StatName::Health, 900)],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::BlackHoleGauntlet,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [black_hole_gauntlet_ranged_min, zero],
    melee: [black_hole_gauntlet_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 447122,
    identifiers: [
        [&[SteelcapsEffect] as &[_], &[]],
        [&[SteelcapsEffect] as &[_], &[]],
    ],
};

pub const fn black_hole_gauntlet_melee_min(ctx: &Ctx) -> f32 {
    5.1 * ctx.steelcaps_effect
}





pub const fn black_hole_gauntlet_ranged_min(ctx: &Ctx) -> f32 {
    5.1 * ctx.steelcaps_effect
}







pub static BLACK_SPEAR: Item = Item {
    name: "Black Spear",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::BlackSpear,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3599,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static BLACKFIRE_TORCH: Item = Item {
    name: "Blackfire Torch",
    tier: 3,
    price: 2800,
    stats: &[
        (StatName::AbilityHaste, 20),
        (StatName::AbilityPower, 80),
        (StatName::Mana, 600),
    ],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::BlackfireTorch,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2503,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static BLADE_OF_THE_RUINED_KING: Item = Item {
    name: "Blade of the Ruined King",
    tier: 3,
    price: 3200,
    stats: &[
        (StatName::AttackDamage, 40),
        (StatName::AttackSpeed, 25),
        (StatName::LifeSteal, 10),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::BladeOfTheRuinedKing,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3153,
    identifiers: [
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
    ],
};












pub static BLASTING_WAND: Item = Item {
    name: "Blasting Wand",
    tier: 1,
    price: 850,
    stats: &[(StatName::AbilityPower, 45)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::BlastingWand,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1026,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static BLIGHTING_JEWEL: Item = Item {
    name: "Blighting Jewel",
    tier: 2,
    price: 1100,
    stats: &[
        (StatName::AbilityPower, 25),
        (StatName::MagicPenetration, 13),
    ],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::BlightingJewel,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 4630,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static BLOODLETTERS_CURSE: Item = Item {
    name: "Bloodletter's Curse",
    tier: 3,
    price: 2900,
    stats: &[
        (StatName::AbilityHaste, 15),
        (StatName::AbilityPower, 65),
        (StatName::Health, 400),
    ],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::BloodlettersCurse,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [bloodletters_curse_ranged_min, zero],
    melee: [bloodletters_curse_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 8010,
    identifiers: [
        [&[MagicResist, SteelcapsEffect] as &[_], &[]],
        [&[MagicResist, SteelcapsEffect] as &[_], &[]],
    ],
};

pub const fn bloodletters_curse_melee_min(ctx: &Ctx) -> f32 {
    0.075 * ctx.magic_resist + 0.3 * ctx.steelcaps_effect
}





pub const fn bloodletters_curse_ranged_min(ctx: &Ctx) -> f32 {
    0.075 * ctx.magic_resist + 0.3 * ctx.steelcaps_effect
}







pub static BLOODSONG: Item = Item {
    name: "Bloodsong",
    tier: 3,
    price: 400,
    stats: &[
        (StatName::GoldPer10Seconds, 9),
        (StatName::Health, 200),
        (StatName::BaseHealthRegen, 75),
        (StatName::BaseManaRegen, 75),
    ],
    maps: &[SummonersRift],
    metadata: TypeMetadata {
        kind: ItemId::Bloodsong,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3877,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static BLOODTHIRSTER: Item = Item {
    name: "Bloodthirster",
    tier: 3,
    price: 3400,
    stats: &[(StatName::AttackDamage, 80), (StatName::LifeSteal, 15)],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Bloodthirster,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3072,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static BOOTS: Item = Item {
    name: "Boots",
    tier: 1,
    price: 300,
    stats: &[(StatName::MoveSpeedPercent, 25)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Boots,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1001,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static BOOTS_OF_SWIFTNESS: Item = Item {
    name: "Boots of Swiftness",
    tier: 2,
    price: 1000,
    stats: &[(StatName::MoveSpeedPercent, 55)],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::BootsOfSwiftness,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3009,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static BOUNTY_OF_WORLDS: Item = Item {
    name: "Bounty of Worlds",
    tier: 3,
    price: 400,
    stats: &[
        (StatName::GoldPer10Seconds, 5),
        (StatName::Health, 200),
        (StatName::BaseHealthRegen, 75),
        (StatName::BaseManaRegen, 75),
    ],
    maps: &[SummonersRift],
    metadata: TypeMetadata {
        kind: ItemId::BountyOfWorlds,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3867,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static BRAMBLE_VEST: Item = Item {
    name: "Bramble Vest",
    tier: 2,
    price: 800,
    stats: &[(StatName::Armor, 30)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::BrambleVest,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [bramble_vest_ranged_min, zero],
    melee: [bramble_vest_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3076,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn bramble_vest_melee_min(_: &Ctx) -> f32 {
    10f32
}





pub const fn bramble_vest_ranged_min(_: &Ctx) -> f32 {
    10f32
}







pub static BRAVERY_VOUCHER: Item = Item {
    name: "Bravery Voucher",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::BraveryVoucher,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 9999,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static CAPPA_JUICE: Item = Item {
    name: "Cappa Juice",
    tier: 1,
    price: 300,
    stats: &[],
    maps: &[Arena, Aram],
    metadata: TypeMetadata {
        kind: ItemId::CappaJuice,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2141,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static CATALYST_OF_AEONS: Item = Item {
    name: "Catalyst of Aeons",
    tier: 2,
    price: 1300,
    stats: &[(StatName::Health, 300), (StatName::Mana, 375)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::CatalystOfAeons,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3803,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static CAULFIELDS_WARHAMMER: Item = Item {
    name: "Caulfield's Warhammer",
    tier: 2,
    price: 1050,
    stats: &[(StatName::AttackDamage, 20), (StatName::AbilityHaste, 10)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::CaulfieldsWarhammer,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3133,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static CELESTIAL_OPPOSITION: Item = Item {
    name: "Celestial Opposition",
    tier: 3,
    price: 400,
    stats: &[
        (StatName::GoldPer10Seconds, 9),
        (StatName::Health, 200),
        (StatName::BaseHealthRegen, 75),
        (StatName::BaseManaRegen, 75),
    ],
    maps: &[SummonersRift],
    metadata: TypeMetadata {
        kind: ItemId::CelestialOpposition,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3869,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static CHAIN_VEST: Item = Item {
    name: "Chain Vest",
    tier: 2,
    price: 800,
    stats: &[(StatName::Armor, 40)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ChainVest,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1031,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static CHAINLACED_CRUSHERS: Item = Item {
    name: "Chainlaced Crushers",
    tier: 3,
    price: 1250,
    stats: &[
        (StatName::MagicResist, 30),
        (StatName::MoveSpeedPercent, 45),
        (StatName::Tenacity, 30),
    ],
    maps: &[SummonersRift],
    metadata: TypeMetadata {
        kind: ItemId::ChainlacedCrushers,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3173,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static CHEMPUNK_CHAINSWORD: Item = Item {
    name: "Chempunk Chainsword",
    tier: 3,
    price: 3000,
    stats: &[
        (StatName::AttackDamage, 45),
        (StatName::AbilityHaste, 15),
        (StatName::Health, 450),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ChempunkChainsword,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 6609,
    identifiers: [
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
    ],
};

pub const fn chempunk_chainsword_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn chempunk_chainsword_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static CLOAK_OF_AGILITY: Item = Item {
    name: "Cloak of Agility",
    tier: 1,
    price: 600,
    stats: &[(StatName::CritChance, 15)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::CloakOfAgility,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1018,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static CLOAK_OF_STARRY_NIGHT: Item = Item {
    name: "Cloak of Starry Night",
    tier: 3,
    price: 0,
    stats: &[(StatName::Health, 300), (StatName::MagicResist, 100)],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::CloakOfStarryNight,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [cloak_of_starry_night_ranged_min, zero],
    melee: [cloak_of_starry_night_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 443059,
    identifiers: [
        [&[SteelcapsEffect] as &[_], &[]],
        [&[SteelcapsEffect] as &[_], &[]],
    ],
};

pub const fn cloak_of_starry_night_melee_min(ctx: &Ctx) -> f32 {
    200f32 + 0.2 * ctx.steelcaps_effect
}





pub const fn cloak_of_starry_night_ranged_min(ctx: &Ctx) -> f32 {
    200f32 + 0.2 * ctx.steelcaps_effect
}







pub static CLOTH_ARMOR: Item = Item {
    name: "Cloth Armor",
    tier: 1,
    price: 300,
    stats: &[(StatName::Armor, 15)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ClothArmor,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1029,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static CONTROL_WARD: Item = Item {
    name: "Control Ward",
    tier: 1,
    price: 75,
    stats: &[],
    maps: &[SummonersRift],
    metadata: TypeMetadata {
        kind: ItemId::ControlWard,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2055,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static COSMIC_DRIVE: Item = Item {
    name: "Cosmic Drive",
    tier: 3,
    price: 3000,
    stats: &[
        (StatName::AbilityHaste, 25),
        (StatName::AbilityPower, 70),
        (StatName::Health, 350),
        (StatName::MoveSpeed, 4),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::CosmicDrive,
        damage_type: True,
        attributes: Undefined,
    },
    ranged: [cosmic_drive_ranged_min, zero],
    melee: [cosmic_drive_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 4629,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn cosmic_drive_melee_min(_: &Ctx) -> f32 {
    20f32
}





pub const fn cosmic_drive_ranged_min(_: &Ctx) -> f32 {
    20f32
}







pub static CRIMSON_LUCIDITY: Item = Item {
    name: "Crimson Lucidity",
    tier: 3,
    price: 900,
    stats: &[
        (StatName::AbilityHaste, 20),
        (StatName::MoveSpeedPercent, 45),
    ],
    maps: &[SummonersRift],
    metadata: TypeMetadata {
        kind: ItemId::CrimsonLucidity,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3171,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static CROWN_OF_THE_SHATTERED_QUEEN: Item = Item {
    name: "Crown of the Shattered Queen",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AbilityHaste, 25),
        (StatName::AbilityPower, 85),
        (StatName::Health, 350),
        (StatName::Mana, 600),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::CrownOfTheShatteredQueen,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 444644,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn crown_of_the_shattered_queen_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn crown_of_the_shattered_queen_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static CRUELTY: Item = Item {
    name: "Cruelty",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AbilityPower, 80),
        (StatName::Armor, 30),
        (StatName::MagicResist, 30),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::Cruelty,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 447109,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static CRYPTBLOOM: Item = Item {
    name: "Cryptbloom",
    tier: 3,
    price: 3000,
    stats: &[
        (StatName::AbilityHaste, 20),
        (StatName::AbilityPower, 75),
        (StatName::MagicPenetration, 30),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Cryptbloom,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3137,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static CRYSTALLINE_BRACER: Item = Item {
    name: "Crystalline Bracer",
    tier: 2,
    price: 800,
    stats: &[(StatName::Health, 200), (StatName::BaseHealthRegen, 100)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::CrystallineBracer,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3801,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static CRYSTALLINE_OVERGROWTH: Item = Item {
    name: "Crystalline Overgrowth",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[SummonersRift],
    metadata: TypeMetadata {
        kind: ItemId::CrystallineOvergrowth,
        damage_type: True,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 1524,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn crystalline_overgrowth_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn crystalline_overgrowth_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static CULL: Item = Item {
    name: "Cull",
    tier: 1,
    price: 450,
    stats: &[(StatName::AttackDamage, 7)],
    maps: &[SummonersRift],
    metadata: TypeMetadata {
        kind: ItemId::Cull,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1083,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static DAGGER: Item = Item {
    name: "Dagger",
    tier: 1,
    price: 250,
    stats: &[(StatName::AttackSpeed, 10)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Dagger,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1042,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static DARK_SEAL: Item = Item {
    name: "Dark Seal",
    tier: 1,
    price: 350,
    stats: &[(StatName::AbilityPower, 15), (StatName::Health, 50)],
    maps: &[SummonersRift],
    metadata: TypeMetadata {
        kind: ItemId::DarkSeal,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1082,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static DARKSTEEL_TALONS: Item = Item {
    name: "Darksteel Talons",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::Armor, 55),
        (StatName::AttackSpeed, 50),
        (StatName::MoveSpeed, 5),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::DarksteelTalons,
        damage_type: True,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 443054,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static DAWNCORE: Item = Item {
    name: "Dawncore",
    tier: 3,
    price: 2500,
    stats: &[
        (StatName::AbilityPower, 45),
        (StatName::HealAndShieldPower, 16),
        (StatName::BaseManaRegen, 100),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Dawncore,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 6621,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static DEAD_MANS_PLATE: Item = Item {
    name: "Dead Man's Plate",
    tier: 3,
    price: 2900,
    stats: &[
        (StatName::Armor, 55),
        (StatName::Health, 350),
        (StatName::MoveSpeed, 4),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::DeadMansPlate,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3742,
    identifiers: [
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
    ],
};












pub static DEATHS_DANCE: Item = Item {
    name: "Death's Dance",
    tier: 3,
    price: 3300,
    stats: &[
        (StatName::AttackDamage, 60),
        (StatName::AbilityHaste, 15),
        (StatName::Armor, 50),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::DeathsDance,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 6333,
    identifiers: [
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
    ],
};












pub static DEATHS_DAUGHTER: Item = Item {
    name: "Death's Daughter",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::DeathsDaughter,
        damage_type: True,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3902,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static DECAPITATOR: Item = Item {
    name: "Decapitator",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AttackSpeed, 50),
        (StatName::MoveSpeed, 8),
        (StatName::AdaptiveForce, 80),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::Decapitator,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 447107,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};


pub const fn decapitator_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn decapitator_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static DEMON_KINGS_CROWN: Item = Item {
    name: "Demon King's Crown",
    tier: 3,
    price: 0,
    stats: &[],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::DemonKingsCrown,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 443056,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static DEMONIC_EMBRACE: Item = Item {
    name: "Demonic Embrace",
    tier: 3,
    price: 0,
    stats: &[(StatName::AbilityPower, 80), (StatName::Health, 700)],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::DemonicEmbrace,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 444637,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static DETONATION_ORB: Item = Item {
    name: "Detonation Orb",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AbilityHaste, 20),
        (StatName::AbilityPower, 90),
        (StatName::Mana, 600),
        (StatName::MagicPenetration, 12),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::DetonationOrb,
        damage_type: True,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 447113,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn detonation_orb_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn detonation_orb_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static DIADEM_OF_SONGS: Item = Item {
    name: "Diadem of Songs",
    tier: 4,
    price: 2250,
    stats: &[
        (StatName::Health, 200),
        (StatName::HealAndShieldPower, 8),
        (StatName::Mana, 1000),
        (StatName::BaseManaRegen, 100),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::DiademOfSongs,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2530,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static DIAMOND_TIPPED_SPEAR: Item = Item {
    name: "Diamond-Tipped Spear",
    tier: 3,
    price: 0,
    stats: &[(StatName::AttackSpeed, 30), (StatName::AdaptiveForce, 75)],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::DiamondTippedSpear,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 447120,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static DIVINE_SUNDERER: Item = Item {
    name: "Divine Sunderer",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AttackDamage, 55),
        (StatName::AbilityHaste, 20),
        (StatName::Health, 350),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::DivineSunderer,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 446632,
    identifiers: [
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
    ],
};












pub static DORANS_BLADE: Item = Item {
    name: "Doran's Blade",
    tier: 1,
    price: 450,
    stats: &[
        (StatName::AttackDamage, 10),
        (StatName::Health, 80),
        (StatName::Omnivamp, 2),
    ],
    maps: &[SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::DoransBlade,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1055,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static DORANS_BOW: Item = Item {
    name: "Doran's Bow",
    tier: 1,
    price: 400,
    stats: &[
        (StatName::AttackDamage, 8),
        (StatName::AttackSpeed, 15),
        (StatName::Omnivamp, 1),
    ],
    maps: &[SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::DoransBow,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1086,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static DORANS_HELM: Item = Item {
    name: "Doran's Helm",
    tier: 1,
    price: 450,
    stats: &[
        (StatName::Armor, 10),
        (StatName::Health, 140),
        (StatName::MagicResist, 10),
    ],
    maps: &[SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::DoransHelm,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [dorans_helm_ranged_min, zero],
    melee: [dorans_helm_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 1120,
    identifiers: [
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
    ],
};

pub const fn dorans_helm_melee_min(_: &Ctx) -> f32 {
    5f32
}





pub const fn dorans_helm_ranged_min(_: &Ctx) -> f32 {
    5f32
}







pub static DORANS_RING: Item = Item {
    name: "Doran's Ring",
    tier: 1,
    price: 400,
    stats: &[(StatName::AbilityPower, 18), (StatName::Health, 90)],
    maps: &[SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::DoransRing,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1056,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static DORANS_SHIELD: Item = Item {
    name: "Doran's Shield",
    tier: 1,
    price: 450,
    stats: &[(StatName::Health, 110), (StatName::BaseHealthRegen, 4)],
    maps: &[SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::DoransShield,
        damage_type: True,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 1054,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn dorans_shield_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn dorans_shield_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static DRAGONHEART: Item = Item {
    name: "Dragonheart",
    tier: 3,
    price: 0,
    stats: &[],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::Dragonheart,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 447106,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static DREAM_MAKER: Item = Item {
    name: "Dream Maker",
    tier: 3,
    price: 400,
    stats: &[
        (StatName::GoldPer10Seconds, 9),
        (StatName::Health, 200),
        (StatName::BaseHealthRegen, 75),
        (StatName::BaseManaRegen, 75),
    ],
    maps: &[SummonersRift],
    metadata: TypeMetadata {
        kind: ItemId::DreamMaker,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3870,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static DUSK_AND_DAWN: Item = Item {
    name: "Dusk and Dawn",
    tier: 3,
    price: 3100,
    stats: &[
        (StatName::AbilityHaste, 20),
        (StatName::AbilityPower, 60),
        (StatName::AttackSpeed, 20),
        (StatName::Health, 300),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::DuskAndDawn,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [dusk_and_dawn_ranged_min, zero],
    melee: [dusk_and_dawn_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 2510,
    identifiers: [
        [&[AbilityPower, AttackDamage, BonusHealth] as &[_], &[]],
        [&[AbilityPower, AttackDamage, BonusHealth] as &[_], &[]],
    ],
};

pub const fn dusk_and_dawn_melee_min(ctx: &Ctx) -> f32 {
    0.1 * ctx.ability_power + 0.75 * ctx.attack_damage + 0.03 * ctx.bonus_health
}





pub const fn dusk_and_dawn_ranged_min(ctx: &Ctx) -> f32 {
    0.1 * ctx.ability_power + 0.75 * ctx.attack_damage + 0.03 * ctx.bonus_health
}







pub static DUSKBLADE_OF_DRAKTHARR: Item = Item {
    name: "Duskblade of Draktharr",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AttackDamage, 50),
        (StatName::AbilityHaste, 20),
        (StatName::Lethality, 20),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::DuskbladeOfDraktharr,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 446691,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn duskblade_of_draktharr_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn duskblade_of_draktharr_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static ECHOES_OF_HELIA: Item = Item {
    name: "Echoes of Helia",
    tier: 3,
    price: 2200,
    stats: &[
        (StatName::AbilityHaste, 20),
        (StatName::AbilityPower, 35),
        (StatName::Health, 200),
        (StatName::BaseManaRegen, 125),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::EchoesOfHelia,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 6620,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static ECLIPSE: Item = Item {
    name: "Eclipse",
    tier: 3,
    price: 2900,
    stats: &[(StatName::AttackDamage, 60), (StatName::AbilityHaste, 15)],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Eclipse,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 6692,
    identifiers: [
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
    ],
};












pub static EDGE_OF_NIGHT: Item = Item {
    name: "Edge of Night",
    tier: 3,
    price: 3000,
    stats: &[
        (StatName::AttackDamage, 50),
        (StatName::Health, 250),
        (StatName::Lethality, 15),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::EdgeOfNight,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3814,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn edge_of_night_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn edge_of_night_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static ELEISAS_MIRACLE: Item = Item {
    name: "Eleisa's Miracle",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AbilityHaste, 25),
        (StatName::Armor, 50),
        (StatName::MagicResist, 50),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::EleisasMiracle,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 443063,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static ELIXIR_OF_AVARICE: Item = Item {
    name: "Elixir of Avarice",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ElixirOfAvarice,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2151,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static ELIXIR_OF_FORCE: Item = Item {
    name: "Elixir of Force",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ElixirOfForce,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2152,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static ELIXIR_OF_IRON: Item = Item {
    name: "Elixir of Iron",
    tier: 1,
    price: 500,
    stats: &[],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ElixirOfIron,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2138,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static ELIXIR_OF_SKILL: Item = Item {
    name: "Elixir of Skill",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ElixirOfSkill,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2150,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static ELIXIR_OF_SORCERY: Item = Item {
    name: "Elixir of Sorcery",
    tier: 1,
    price: 500,
    stats: &[],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ElixirOfSorcery,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2139,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static ELIXIR_OF_WRATH: Item = Item {
    name: "Elixir of Wrath",
    tier: 1,
    price: 500,
    stats: &[],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ElixirOfWrath,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2140,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static EMPYREAN_PROMISE: Item = Item {
    name: "Empyrean Promise",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AbilityHaste, 30),
        (StatName::AbilityPower, 70),
        (StatName::HealAndShieldPower, 18),
        (StatName::BaseManaRegen, 125),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::EmpyreanPromise,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 447105,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static ENDLESS_HUNGER: Item = Item {
    name: "Endless Hunger",
    tier: 3,
    price: 3100,
    stats: &[
        (StatName::AttackDamage, 65),
        (StatName::Omnivamp, 5),
        (StatName::Tenacity, 20),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::EndlessHunger,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2517,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static ENHANCED_LUCKY_DICE: Item = Item {
    name: "Enhanced Lucky Dice",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::EnhancedLuckyDice,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2146,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static ESSENCE_REAVER: Item = Item {
    name: "Essence Reaver",
    tier: 3,
    price: 3050,
    stats: &[
        (StatName::AttackDamage, 50),
        (StatName::AbilityHaste, 20),
        (StatName::CritChance, 25),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::EssenceReaver,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3508,
    identifiers: [
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
    ],
};












pub static EVERFROST: Item = Item {
    name: "Everfrost",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AbilityHaste, 25),
        (StatName::AbilityPower, 100),
        (StatName::Health, 250),
        (StatName::Mana, 600),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::Everfrost,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [everfrost_ranged_min, zero],
    melee: [everfrost_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 446656,
    identifiers: [
        [&[AbilityPower] as &[_], &[]],
        [&[AbilityPower] as &[_], &[]],
    ],
};


pub const fn everfrost_melee_min(ctx: &Ctx) -> f32 {
    300f32 + 0.85 * ctx.ability_power
}





pub const fn everfrost_ranged_min(ctx: &Ctx) -> f32 {
    300f32 + 0.85 * ctx.ability_power
}







pub static EXECUTIONERS_CALLING: Item = Item {
    name: "Executioner's Calling",
    tier: 2,
    price: 800,
    stats: &[(StatName::AttackDamage, 15)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ExecutionersCalling,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3123,
    identifiers: [
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
    ],
};

pub const fn executioners_calling_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn executioners_calling_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static EXPERIMENTAL_HEXPLATE: Item = Item {
    name: "Experimental Hexplate",
    tier: 3,
    price: 3000,
    stats: &[
        (StatName::AttackDamage, 40),
        (StatName::AttackSpeed, 20),
        (StatName::Health, 450),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ExperimentalHexplate,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3073,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static EYE_OF_THE_HERALD: Item = Item {
    name: "Eye of the Herald",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::EyeOfTheHerald,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3513,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static FAERIE_CHARM: Item = Item {
    name: "Faerie Charm",
    tier: 1,
    price: 200,
    stats: &[(StatName::BaseManaRegen, 50)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::FaerieCharm,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1004,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static FARSIGHT_ALTERATION: Item = Item {
    name: "Farsight Alteration",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::FarsightAlteration,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3363,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static FATED_ASHES: Item = Item {
    name: "Fated Ashes",
    tier: 2,
    price: 900,
    stats: &[(StatName::AbilityPower, 30)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::FatedAshes,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2508,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static FIENDHUNTER_BOLTS: Item = Item {
    name: "Fiendhunter Bolts",
    tier: 3,
    price: 2650,
    stats: &[
        (StatName::AttackSpeed, 45),
        (StatName::CritChance, 25),
        (StatName::MoveSpeed, 4),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::FiendhunterBolts,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2512,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static FIENDISH_CODEX: Item = Item {
    name: "Fiendish Codex",
    tier: 2,
    price: 850,
    stats: &[(StatName::AbilityHaste, 10), (StatName::AbilityPower, 25)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::FiendishCodex,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3108,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static FIMBULWINTER: Item = Item {
    name: "Fimbulwinter",
    tier: 4,
    price: 2400,
    stats: &[
        (StatName::AbilityHaste, 15),
        (StatName::Health, 550),
        (StatName::Mana, 1000),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Fimbulwinter,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3121,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static FIRE_AT_WILL: Item = Item {
    name: "Fire at Will",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::FireAtWill,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3901,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static FLESHEATER: Item = Item {
    name: "Flesheater",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AbilityHaste, 20),
        (StatName::Health, 500),
        (StatName::AdaptiveForce, 70),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::Flesheater,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 447112,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};


pub const fn flesheater_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn flesheater_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static FORBIDDEN_IDOL: Item = Item {
    name: "Forbidden Idol",
    tier: 2,
    price: 600,
    stats: &[
        (StatName::HealAndShieldPower, 8),
        (StatName::BaseManaRegen, 50),
    ],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ForbiddenIdol,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3114,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static FORCE_OF_ENTROPY: Item = Item {
    name: "Force of Entropy",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AbilityHaste, 30),
        (StatName::CritChance, 25),
        (StatName::Health, 900),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::ForceOfEntropy,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 443061,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static FORCE_OF_NATURE: Item = Item {
    name: "Force of Nature",
    tier: 3,
    price: 2800,
    stats: &[
        (StatName::Health, 400),
        (StatName::MagicResist, 55),
        (StatName::MoveSpeed, 4),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ForceOfNature,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [force_of_nature_ranged_min, zero],
    melee: [force_of_nature_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 4401,
    identifiers: [
        [&[BonusMoveSpeed] as &[_], &[]],
        [&[BonusMoveSpeed] as &[_], &[]],
    ],
};

pub const fn force_of_nature_melee_min(ctx: &Ctx) -> f32 {
    70f32 + 0.06 * ctx.bonus_move_speed
}





pub const fn force_of_nature_ranged_min(ctx: &Ctx) -> f32 {
    70f32 + 0.06 * ctx.bonus_move_speed
}







pub static FORTIFICATION_ARAM: Item = Item {
    name: "Fortification (ARAM)",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Aram],
    metadata: TypeMetadata {
        kind: ItemId::FortificationAram,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 999999,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn fortification_aram_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn fortification_aram_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static FROZEN_HEART: Item = Item {
    name: "Frozen Heart",
    tier: 3,
    price: 2500,
    stats: &[
        (StatName::AbilityHaste, 20),
        (StatName::Armor, 75),
        (StatName::Mana, 400),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::FrozenHeart,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3110,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static FULMINATION: Item = Item {
    name: "Fulmination",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AttackDamage, 55),
        (StatName::AttackSpeed, 45),
        (StatName::MoveSpeed, 15),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::Fulmination,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 443055,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static GALEFORCE: Item = Item {
    name: "Galeforce",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AttackDamage, 65),
        (StatName::AttackSpeed, 30),
        (StatName::CritChance, 25),
        (StatName::MoveSpeed, 6),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::Galeforce,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 446671,
    identifiers: [
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
    ],
};












pub static GAMBLERS_BLADE: Item = Item {
    name: "Gambler's Blade",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AbilityHaste, 40),
        (StatName::AttackSpeed, 70),
        (StatName::MoveSpeed, 8),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::GamblersBlade,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 447101,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static GARGOYLE_STONEPLATE: Item = Item {
    name: "Gargoyle Stoneplate",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AbilityHaste, 15),
        (StatName::Armor, 65),
        (StatName::MagicResist, 65),
        (StatName::MoveSpeed, 10),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::GargoyleStoneplate,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 443193,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static GHOSTCRAWLERS: Item = Item {
    name: "Ghostcrawlers",
    tier: 2,
    price: 500,
    stats: &[(StatName::MoveSpeedPercent, 70)],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::Ghostcrawlers,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 223005,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static GIANTS_BELT: Item = Item {
    name: "Giant's Belt",
    tier: 2,
    price: 900,
    stats: &[(StatName::Health, 350)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::GiantsBelt,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1011,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static GLACIAL_BUCKLER: Item = Item {
    name: "Glacial Buckler",
    tier: 2,
    price: 900,
    stats: &[
        (StatName::AbilityHaste, 10),
        (StatName::Armor, 25),
        (StatName::Mana, 300),
    ],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::GlacialBuckler,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3024,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static GLOWING_MOTE: Item = Item {
    name: "Glowing Mote",
    tier: 1,
    price: 250,
    stats: &[(StatName::AbilityHaste, 5)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::GlowingMote,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2022,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static GLUTTONOUS_GREAVES: Item = Item {
    name: "Gluttonous Greaves",
    tier: 2,
    price: 1000,
    stats: &[(StatName::MoveSpeedPercent, 45), (StatName::Omnivamp, 4)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::GluttonousGreaves,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3008,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static GOLD_STAT_ANVIL_VOUCHER: Item = Item {
    name: "Gold Stat Anvil Voucher",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::GoldStatAnvilVoucher,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 9999,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static GOREDRINKER: Item = Item {
    name: "Goredrinker",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AttackDamage, 55),
        (StatName::AbilityHaste, 20),
        (StatName::Health, 400),
        (StatName::Omnivamp, 10),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::Goredrinker,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [goredrinker_ranged_min, zero],
    melee: [goredrinker_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 226630,
    identifiers: [
        [
            &[AttackDamage, MissingHealth, PhysicalMultiplier] as &[_],
            &[PhysicalMultiplier],
        ],
        [
            &[AttackDamage, MissingHealth, PhysicalMultiplier] as &[_],
            &[PhysicalMultiplier],
        ],
    ],
};


pub const fn goredrinker_melee_min(ctx: &Ctx) -> f32 {
    (2.1 * ctx.attack_damage) + 0.18 * ctx.missing_health
}





pub const fn goredrinker_ranged_min(ctx: &Ctx) -> f32 {
    (2.1 * ctx.attack_damage) + 0.18 * ctx.missing_health
}







pub static GUARDIAN_ANGEL: Item = Item {
    name: "Guardian Angel",
    tier: 3,
    price: 3200,
    stats: &[(StatName::AttackDamage, 55), (StatName::Armor, 45)],
    maps: &[Arena, SummonersRift],
    metadata: TypeMetadata {
        kind: ItemId::GuardianAngel,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [guardian_angel_ranged_min, zero],
    melee: [guardian_angel_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3026,
    identifiers: [
        [&[MaxMana, SteelcapsEffect] as &[_], &[]],
        [&[MaxMana, SteelcapsEffect] as &[_], &[]],
    ],
};

pub const fn guardian_angel_melee_min(ctx: &Ctx) -> f32 {
    (2f32 * ctx.max_mana + ctx.steelcaps_effect) / 2f32
}





pub const fn guardian_angel_ranged_min(ctx: &Ctx) -> f32 {
    (2f32 * ctx.max_mana + ctx.steelcaps_effect) / 2f32
}







pub static GUARDIANS_AMULET: Item = Item {
    name: "Guardian's Amulet",
    tier: 1,
    price: 500,
    stats: &[
        (StatName::AbilityHaste, 20),
        (StatName::AbilityPower, 20),
        (StatName::HealAndShieldPower, 15),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::GuardiansAmulet,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2049,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static GUARDIANS_BLADE: Item = Item {
    name: "Guardian's Blade",
    tier: 1,
    price: 950,
    stats: &[
        (StatName::AttackDamage, 30),
        (StatName::AbilityHaste, 15),
        (StatName::Health, 150),
    ],
    maps: &[Arena, Aram],
    metadata: TypeMetadata {
        kind: ItemId::GuardiansBlade,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3177,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static GUARDIANS_DIRK: Item = Item {
    name: "Guardian's Dirk",
    tier: 1,
    price: 500,
    stats: &[
        (StatName::AttackDamage, 25),
        (StatName::AbilityHaste, 10),
        (StatName::Lethality, 11),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::GuardiansDirk,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [guardians_dirk_ranged_min, zero],
    melee: [guardians_dirk_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 223185,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn guardians_dirk_melee_min(_: &Ctx) -> f32 {
    100f32
}





pub const fn guardians_dirk_ranged_min(_: &Ctx) -> f32 {
    100f32
}







pub static GUARDIANS_HAMMER: Item = Item {
    name: "Guardian's Hammer",
    tier: 1,
    price: 950,
    stats: &[
        (StatName::AttackDamage, 25),
        (StatName::Health, 150),
        (StatName::LifeSteal, 5),
    ],
    maps: &[Arena, Aram],
    metadata: TypeMetadata {
        kind: ItemId::GuardiansHammer,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3184,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static GUARDIANS_HORN: Item = Item {
    name: "Guardian's Horn",
    tier: 1,
    price: 950,
    stats: &[(StatName::Health, 150), (StatName::BaseHealthRegen, 20)],
    maps: &[Arena, Aram],
    metadata: TypeMetadata {
        kind: ItemId::GuardiansHorn,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 2051,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn guardians_horn_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn guardians_horn_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static GUARDIANS_ORB: Item = Item {
    name: "Guardian's Orb",
    tier: 1,
    price: 950,
    stats: &[(StatName::AbilityPower, 50), (StatName::Health, 150)],
    maps: &[Arena, Aram],
    metadata: TypeMetadata {
        kind: ItemId::GuardiansOrb,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3112,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static GUARDIANS_SHROUD: Item = Item {
    name: "Guardian's Shroud",
    tier: 1,
    price: 500,
    stats: &[
        (StatName::AbilityHaste, 15),
        (StatName::AbilityPower, 35),
        (StatName::Health, 300),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::GuardiansShroud,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2050,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static GUINSOOS_RAGEBLADE: Item = Item {
    name: "Guinsoo's Rageblade",
    tier: 3,
    price: 3000,
    stats: &[
        (StatName::AttackDamage, 30),
        (StatName::AbilityPower, 30),
        (StatName::AttackSpeed, 25),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::GuinsoosRageblade,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [guinsoos_rageblade_ranged_min, zero],
    melee: [guinsoos_rageblade_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3124,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn guinsoos_rageblade_melee_min(_: &Ctx) -> f32 {
    30f32
}





pub const fn guinsoos_rageblade_ranged_min(_: &Ctx) -> f32 {
    30f32
}







pub static GUNMETAL_GREAVES: Item = Item {
    name: "Gunmetal Greaves",
    tier: 3,
    price: 1100,
    stats: &[
        (StatName::AttackSpeed, 40),
        (StatName::LifeSteal, 5),
        (StatName::MoveSpeedPercent, 45),
    ],
    maps: &[SummonersRift],
    metadata: TypeMetadata {
        kind: ItemId::GunmetalGreaves,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3172,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static GUSTO: Item = Item {
    name: "Gusto",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Gusto,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [gusto_ranged_min, zero],
    melee: [gusto_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 1509,
    identifiers: [[&[MaxHealth] as &[_], &[]], [&[MaxHealth] as &[_], &[]]],
};


pub const fn gusto_melee_min(ctx: &Ctx) -> f32 {
    0.45 * ctx.max_health
}





pub const fn gusto_ranged_min(ctx: &Ctx) -> f32 {
    0.45 * ctx.max_health
}







pub static GUSTWALKER_HATCHLING: Item = Item {
    name: "Gustwalker Hatchling",
    tier: 1,
    price: 450,
    stats: &[],
    maps: &[SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::GustwalkerHatchling,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1102,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static HAMSTRINGER: Item = Item {
    name: "Hamstringer",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AttackDamage, 45),
        (StatName::AttackSpeed, 40),
        (StatName::CritChance, 25),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::Hamstringer,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 443069,
    identifiers: [
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
    ],
};












pub static HAUNTING_GUISE: Item = Item {
    name: "Haunting Guise",
    tier: 2,
    price: 1300,
    stats: &[(StatName::AbilityPower, 30), (StatName::Health, 200)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::HauntingGuise,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3147,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn haunting_guise_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn haunting_guise_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static HEALTH_POTION: Item = Item {
    name: "Health Potion",
    tier: 1,
    price: 50,
    stats: &[],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::HealthPotion,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2003,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static HEARTHBOUND_AXE: Item = Item {
    name: "Hearthbound Axe",
    tier: 2,
    price: 1200,
    stats: &[(StatName::AttackDamage, 20), (StatName::AttackSpeed, 20)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::HearthboundAxe,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3051,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static HEARTSTEEL: Item = Item {
    name: "Heartsteel",
    tier: 3,
    price: 3000,
    stats: &[(StatName::Health, 900), (StatName::BaseHealthRegen, 100)],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Heartsteel,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3084,
    identifiers: [
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
    ],
};












pub static HELLFIRE_HATCHET: Item = Item {
    name: "Hellfire Hatchet",
    tier: 3,
    price: 2500,
    stats: &[(StatName::AttackDamage, 35), (StatName::Lethality, 12)],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::HellfireHatchet,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 4017,
    identifiers: [
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
    ],
};












pub static HEMOMANCERS_HELM: Item = Item {
    name: "Hemomancer's Helm",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AttackDamage, 70),
        (StatName::AbilityHaste, 30),
        (StatName::Omnivamp, 15),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::HemomancersHelm,
        damage_type: True,
        attributes: Undefined,
    },
    ranged: [hemomancers_helm_ranged_min, zero],
    melee: [hemomancers_helm_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 447103,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn hemomancers_helm_melee_min(_: &Ctx) -> f32 {
    500f32
}





pub const fn hemomancers_helm_ranged_min(_: &Ctx) -> f32 {
    500f32
}







pub static HEXBOLT_COMPANION: Item = Item {
    name: "Hexbolt Companion",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AttackSpeed, 75),
        (StatName::Health, 500),
        (StatName::MoveSpeed, 4),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::HexboltCompanion,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 443081,
    identifiers: [
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
    ],
};












pub static HEXDRINKER: Item = Item {
    name: "Hexdrinker",
    tier: 2,
    price: 1300,
    stats: &[(StatName::AttackDamage, 25), (StatName::MagicResist, 25)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Hexdrinker,
        damage_type: True,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3155,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static HEXOPTICS_C_44: Item = Item {
    name: "Hexoptics C44",
    tier: 3,
    price: 2800,
    stats: &[(StatName::AttackDamage, 55), (StatName::CritChance, 25)],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::HexopticsC44,
        damage_type: True,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2523,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static HEXTECH_ALTERNATOR: Item = Item {
    name: "Hextech Alternator",
    tier: 2,
    price: 1100,
    stats: &[(StatName::AbilityPower, 45)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::HextechAlternator,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [hextech_alternator_ranged_min, zero],
    melee: [hextech_alternator_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3145,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn hextech_alternator_melee_min(_: &Ctx) -> f32 {
    65f32
}





pub const fn hextech_alternator_ranged_min(_: &Ctx) -> f32 {
    65f32
}







pub static HEXTECH_GUNBLADE: Item = Item {
    name: "Hextech Gunblade",
    tier: 3,
    price: 3000,
    stats: &[
        (StatName::AttackDamage, 40),
        (StatName::AbilityPower, 80),
        (StatName::Omnivamp, 10),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::HextechGunblade,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3146,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static HEXTECH_ROCKETBELT: Item = Item {
    name: "Hextech Rocketbelt",
    tier: 3,
    price: 2650,
    stats: &[
        (StatName::AbilityHaste, 20),
        (StatName::AbilityPower, 70),
        (StatName::Health, 300),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::HextechRocketbelt,
        damage_type: True,
        attributes: Undefined,
    },
    ranged: [hextech_rocketbelt_ranged_min, zero],
    melee: [hextech_rocketbelt_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3152,
    identifiers: [
        [&[AbilityPower] as &[_], &[]],
        [&[AbilityPower] as &[_], &[]],
    ],
};

pub const fn hextech_rocketbelt_melee_min(ctx: &Ctx) -> f32 {
    100f32 + 0.1 * ctx.ability_power
}





pub const fn hextech_rocketbelt_ranged_min(ctx: &Ctx) -> f32 {
    100f32 + 0.1 * ctx.ability_power
}







pub static HOLLOW_RADIANCE: Item = Item {
    name: "Hollow Radiance",
    tier: 3,
    price: 2800,
    stats: &[
        (StatName::AbilityHaste, 10),
        (StatName::Health, 400),
        (StatName::BaseHealthRegen, 100),
        (StatName::MagicResist, 40),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::HollowRadiance,
        damage_type: True,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 6664,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static HORIZON_FOCUS: Item = Item {
    name: "Horizon Focus",
    tier: 3,
    price: 2700,
    stats: &[(StatName::AbilityHaste, 25), (StatName::AbilityPower, 75)],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::HorizonFocus,
        damage_type: True,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 4628,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn horizon_focus_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn horizon_focus_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static HUBRIS: Item = Item {
    name: "Hubris",
    tier: 3,
    price: 2800,
    stats: &[
        (StatName::AttackDamage, 55),
        (StatName::AbilityHaste, 10),
        (StatName::Lethality, 18),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Hubris,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 6697,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static HULLBREAKER: Item = Item {
    name: "Hullbreaker",
    tier: 3,
    price: 3000,
    stats: &[
        (StatName::AttackDamage, 40),
        (StatName::Health, 500),
        (StatName::MoveSpeed, 4),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Hullbreaker,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3181,
    identifiers: [
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
    ],
};












pub static ICEBORN_GAUNTLET: Item = Item {
    name: "Iceborn Gauntlet",
    tier: 3,
    price: 2900,
    stats: &[
        (StatName::AbilityHaste, 15),
        (StatName::Armor, 50),
        (StatName::Health, 300),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::IcebornGauntlet,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 6662,
    identifiers: [
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
    ],
};












pub static IMMORTAL_PATH: Item = Item {
    name: "Immortal Path",
    tier: 3,
    price: 1000,
    stats: &[(StatName::MoveSpeedPercent, 45), (StatName::Omnivamp, 4)],
    maps: &[SummonersRift],
    metadata: TypeMetadata {
        kind: ItemId::ImmortalPath,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3168,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static IMMORTAL_SHIELDBOW: Item = Item {
    name: "Immortal Shieldbow",
    tier: 3,
    price: 3000,
    stats: &[(StatName::AttackDamage, 55), (StatName::CritChance, 25)],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ImmortalShieldbow,
        damage_type: True,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 6673,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static IMPERIAL_MANDATE: Item = Item {
    name: "Imperial Mandate",
    tier: 3,
    price: 2250,
    stats: &[
        (StatName::AbilityHaste, 20),
        (StatName::AbilityPower, 60),
        (StatName::BaseManaRegen, 125),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ImperialMandate,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [imperial_mandate_ranged_min, zero],
    melee: [imperial_mandate_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 4005,
    identifiers: [
        [&[EnemyCurrentHealth] as &[_], &[]],
        [&[EnemyCurrentHealth] as &[_], &[]],
    ],
};

pub const fn imperial_mandate_melee_min(ctx: &Ctx) -> f32 {
    0.1 * ctx.enemy_current_health
}





pub const fn imperial_mandate_ranged_min(ctx: &Ctx) -> f32 {
    0.1 * ctx.enemy_current_health
}







pub static INFINITY_EDGE: Item = Item {
    name: "Infinity Edge",
    tier: 2,
    price: 3500,
    stats: &[
        (StatName::AttackDamage, 75),
        (StatName::CritChance, 25),
        (StatName::CritDamage, 30),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::InfinityEdge,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3031,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static INNERVATING_LOCKET: Item = Item {
    name: "Innervating Locket",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AbilityHaste, 20),
        (StatName::AbilityPower, 70),
        (StatName::Health, 200),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::InnervatingLocket,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 447104,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static IONIAN_BOOTS_OF_LUCIDITY: Item = Item {
    name: "Ionian Boots of Lucidity",
    tier: 2,
    price: 900,
    stats: &[
        (StatName::AbilityHaste, 10),
        (StatName::MoveSpeedPercent, 45),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::IonianBootsOfLucidity,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3158,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static JAK_SHO_THE_PROTEAN: Item = Item {
    name: "Jak'Sho, The Protean",
    tier: 3,
    price: 3200,
    stats: &[
        (StatName::Armor, 45),
        (StatName::Health, 350),
        (StatName::MagicResist, 45),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::JakShoTheProtean,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 6665,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static JARVAN_IS: Item = Item {
    name: "Jarvan I's",
    tier: 2,
    price: 0,
    stats: &[
        (StatName::AbilityHaste, 10),
        (StatName::Armor, 25),
        (StatName::AttackSpeed, 25),
        (StatName::MagicPenetration, 12),
        (StatName::MagicResist, 20),
        (StatName::MoveSpeedPercent, 100),
        (StatName::Tenacity, 30),
    ],
    maps: &[],
    metadata: TypeMetadata {
        kind: ItemId::JarvanIs,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1111,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static JUICE_OF_HASTE: Item = Item {
    name: "Juice of Haste",
    tier: 1,
    price: 500,
    stats: &[],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::JuiceOfHaste,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2144,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static JUICE_OF_POWER: Item = Item {
    name: "Juice of Power",
    tier: 1,
    price: 500,
    stats: &[],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::JuiceOfPower,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2142,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static JUICE_OF_VITALITY: Item = Item {
    name: "Juice of Vitality",
    tier: 1,
    price: 500,
    stats: &[],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::JuiceOfVitality,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2143,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static KAENIC_ROOKERN: Item = Item {
    name: "Kaenic Rookern",
    tier: 3,
    price: 2900,
    stats: &[
        (StatName::Health, 400),
        (StatName::BaseHealthRegen, 100),
        (StatName::MagicResist, 80),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::KaenicRookern,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [kaenic_rookern_ranged_min, zero],
    melee: [kaenic_rookern_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 2504,
    identifiers: [[&[MaxHealth] as &[_], &[]], [&[MaxHealth] as &[_], &[]]],
};

pub const fn kaenic_rookern_melee_min(ctx: &Ctx) -> f32 {
    0.15 * ctx.max_health
}





pub const fn kaenic_rookern_ranged_min(ctx: &Ctx) -> f32 {
    0.15 * ctx.max_health
}







pub static KINDLEGEM: Item = Item {
    name: "Kindlegem",
    tier: 2,
    price: 800,
    stats: &[(StatName::AbilityHaste, 10), (StatName::Health, 200)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Kindlegem,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3067,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static KINKOU_JITTE: Item = Item {
    name: "Kinkou Jitte",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AbilityHaste, 30),
        (StatName::Health, 400),
        (StatName::AdaptiveForce, 85),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::KinkouJitte,
        damage_type: True,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 447116,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static KNIGHTS_VOW: Item = Item {
    name: "Knight's Vow",
    tier: 3,
    price: 2300,
    stats: &[
        (StatName::AbilityHaste, 10),
        (StatName::Armor, 40),
        (StatName::Health, 200),
        (StatName::BaseHealthRegen, 100),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::KnightsVow,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3109,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static KRAKEN_SLAYER: Item = Item {
    name: "Kraken Slayer",
    tier: 3,
    price: 3000,
    stats: &[
        (StatName::AttackDamage, 45),
        (StatName::AttackSpeed, 40),
        (StatName::MoveSpeed, 4),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::KrakenSlayer,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 6672,
    identifiers: [
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
    ],
};












pub static LAST_WHISPER: Item = Item {
    name: "Last Whisper",
    tier: 2,
    price: 1450,
    stats: &[
        (StatName::AttackDamage, 20),
        (StatName::ArmorPenetration, 18),
    ],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::LastWhisper,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3035,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static LEGENDARY_ASSASSIN_ITEM: Item = Item {
    name: "Legendary Assassin Item",
    tier: 1,
    price: 2000,
    stats: &[],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::LegendaryAssassinItem,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 220003,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static LEGENDARY_FIGHTER_ITEM: Item = Item {
    name: "Legendary Fighter Item",
    tier: 1,
    price: 2000,
    stats: &[],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::LegendaryFighterItem,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 220001,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static LEGENDARY_MAGE_ITEM: Item = Item {
    name: "Legendary Mage Item",
    tier: 1,
    price: 2000,
    stats: &[],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::LegendaryMageItem,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 220004,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static LEGENDARY_MARKSMAN_ITEM: Item = Item {
    name: "Legendary Marksman Item",
    tier: 1,
    price: 2000,
    stats: &[],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::LegendaryMarksmanItem,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 220002,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static LEGENDARY_SUPPORT_ITEM: Item = Item {
    name: "Legendary Support Item",
    tier: 1,
    price: 2000,
    stats: &[],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::LegendarySupportItem,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 220006,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static LEGENDARY_TANK_ITEM: Item = Item {
    name: "Legendary Tank Item",
    tier: 1,
    price: 2000,
    stats: &[],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::LegendaryTankItem,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 220005,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static LIANDRYS_TORMENT: Item = Item {
    name: "Liandry's Torment",
    tier: 3,
    price: 3000,
    stats: &[(StatName::AbilityPower, 60), (StatName::Health, 300)],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::LiandrysTorment,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [liandrys_torment_ranged_min, zero],
    melee: [liandrys_torment_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 6653,
    identifiers: [
        [&[EnemyMaxHealth] as &[_], &[]],
        [&[EnemyMaxHealth] as &[_], &[]],
    ],
};

pub const fn liandrys_torment_melee_min(ctx: &Ctx) -> f32 {
    0.14 * ctx.enemy_max_health
}





pub const fn liandrys_torment_ranged_min(ctx: &Ctx) -> f32 {
    0.14 * ctx.enemy_max_health
}







pub static LICH_BANE: Item = Item {
    name: "Lich Bane",
    tier: 3,
    price: 2900,
    stats: &[
        (StatName::AbilityHaste, 10),
        (StatName::AbilityPower, 100),
        (StatName::MoveSpeed, 6),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::LichBane,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [lich_bane_ranged_min, zero],
    melee: [lich_bane_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3100,
    identifiers: [
        [&[AbilityPower, AttackDamage, AttackSpeed] as &[_], &[]],
        [&[AbilityPower, AttackDamage, AttackSpeed] as &[_], &[]],
    ],
};

pub const fn lich_bane_melee_min(ctx: &Ctx) -> f32 {
    0.45 * ctx.ability_power
        + 0.75 * ctx.attack_damage
        + ctx.attack_speed / 2f32
}





pub const fn lich_bane_ranged_min(ctx: &Ctx) -> f32 {
    0.45 * ctx.ability_power
        + 0.75 * ctx.attack_damage
        + ctx.attack_speed / 2f32
}







pub static LIFELINE: Item = Item {
    name: "Lifeline",
    tier: 2,
    price: 1600,
    stats: &[
        (StatName::AttackDamage, 25),
        (StatName::Lethality, 5),
        (StatName::MoveSpeed, 4),
    ],
    maps: &[Aram, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Lifeline,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 4003,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static LIGHTNING_ROD: Item = Item {
    name: "Lightning Rod",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::Armor, 30),
        (StatName::Health, 500),
        (StatName::MagicResist, 30),
        (StatName::MoveSpeed, 8),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::LightningRod,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 447119,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static LOCKET_OF_THE_IRON_SOLARI: Item = Item {
    name: "Locket of the Iron Solari",
    tier: 3,
    price: 2200,
    stats: &[
        (StatName::AbilityHaste, 10),
        (StatName::Armor, 25),
        (StatName::Health, 200),
        (StatName::MagicResist, 25),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::LocketOfTheIronSolari,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3190,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static LONG_SWORD: Item = Item {
    name: "Long Sword",
    tier: 1,
    price: 350,
    stats: &[(StatName::AttackDamage, 10)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::LongSword,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1036,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static LORD_DOMINIKS_REGARDS: Item = Item {
    name: "Lord Dominik's Regards",
    tier: 3,
    price: 3300,
    stats: &[
        (StatName::AttackDamage, 35),
        (StatName::ArmorPenetration, 35),
        (StatName::CritChance, 25),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::LordDominiksRegards,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3036,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static LOST_CHAPTER: Item = Item {
    name: "Lost Chapter",
    tier: 2,
    price: 1200,
    stats: &[
        (StatName::AbilityHaste, 10),
        (StatName::AbilityPower, 40),
        (StatName::Mana, 300),
    ],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::LostChapter,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3802,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static LUCKY_DICE: Item = Item {
    name: "Lucky Dice",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::LuckyDice,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2145,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static LUDENS_ECHO: Item = Item {
    name: "Luden's Echo",
    tier: 3,
    price: 2750,
    stats: &[
        (StatName::AbilityHaste, 10),
        (StatName::AbilityPower, 100),
        (StatName::Mana, 600),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::LudensEcho,
        damage_type: True,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 6655,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static MALIGNANCE: Item = Item {
    name: "Malignance",
    tier: 3,
    price: 2700,
    stats: &[
        (StatName::AbilityHaste, 15),
        (StatName::AbilityPower, 90),
        (StatName::Mana, 600),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Malignance,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3118,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static MANAMUNE: Item = Item {
    name: "Manamune",
    tier: 3,
    price: 2900,
    stats: &[
        (StatName::AttackDamage, 35),
        (StatName::AbilityHaste, 15),
        (StatName::Mana, 500),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Manamune,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3004,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static MAW_OF_MALMORTIUS: Item = Item {
    name: "Maw of Malmortius",
    tier: 3,
    price: 3100,
    stats: &[
        (StatName::AttackDamage, 60),
        (StatName::AbilityHaste, 15),
        (StatName::MagicResist, 40),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::MawOfMalmortius,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3156,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static MEJAIS_SOULSTEALER: Item = Item {
    name: "Mejai's Soulstealer",
    tier: 2,
    price: 1500,
    stats: &[(StatName::AbilityPower, 20), (StatName::Health, 100)],
    maps: &[SummonersRift],
    metadata: TypeMetadata {
        kind: ItemId::MejaisSoulstealer,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3041,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static MERCURIAL_SCIMITAR: Item = Item {
    name: "Mercurial Scimitar",
    tier: 3,
    price: 3200,
    stats: &[
        (StatName::AttackDamage, 50),
        (StatName::LifeSteal, 10),
        (StatName::MagicResist, 35),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::MercurialScimitar,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3139,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static MERCURYS_TREADS: Item = Item {
    name: "Mercury's Treads",
    tier: 2,
    price: 1250,
    stats: &[
        (StatName::MagicResist, 20),
        (StatName::MoveSpeedPercent, 45),
        (StatName::Tenacity, 30),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::MercurysTreads,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3111,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static MIKAELS_BLESSING: Item = Item {
    name: "Mikael's Blessing",
    tier: 3,
    price: 2300,
    stats: &[
        (StatName::AbilityHaste, 15),
        (StatName::Health, 250),
        (StatName::HealAndShieldPower, 12),
        (StatName::BaseManaRegen, 100),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::MikaelsBlessing,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3222,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static MIRAGE_BLADE: Item = Item {
    name: "Mirage Blade",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AttackSpeed, 60),
        (StatName::MoveSpeed, 6),
        (StatName::AdaptiveForce, 65),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::MirageBlade,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 447100,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static MOONFLAIR_SPELLBLADE: Item = Item {
    name: "Moonflair Spellblade",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AbilityPower, 85),
        (StatName::Health, 400),
        (StatName::Tenacity, 30),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::MoonflairSpellblade,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 447110,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static MOONSTONE_RENEWER: Item = Item {
    name: "Moonstone Renewer",
    tier: 3,
    price: 2200,
    stats: &[
        (StatName::AbilityHaste, 20),
        (StatName::AbilityPower, 25),
        (StatName::Health, 200),
        (StatName::BaseManaRegen, 125),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::MoonstoneRenewer,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 6617,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static MORELLONOMICON: Item = Item {
    name: "Morellonomicon",
    tier: 3,
    price: 2850,
    stats: &[
        (StatName::AbilityHaste, 15),
        (StatName::AbilityPower, 75),
        (StatName::Health, 350),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Morellonomicon,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3165,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};


pub const fn morellonomicon_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn morellonomicon_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static MORTAL_REMINDER: Item = Item {
    name: "Mortal Reminder",
    tier: 3,
    price: 3000,
    stats: &[
        (StatName::AttackDamage, 35),
        (StatName::ArmorPenetration, 30),
        (StatName::CritChance, 25),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::MortalReminder,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3033,
    identifiers: [
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
    ],
};

pub const fn mortal_reminder_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn mortal_reminder_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static MOSSTOMPER_SEEDLING: Item = Item {
    name: "Mosstomper Seedling",
    tier: 1,
    price: 450,
    stats: &[],
    maps: &[SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::MosstomperSeedling,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1103,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static MULTITOOL: Item = Item {
    name: "Multitool",
    tier: 3,
    price: 2500,
    stats: &[],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::Multitool,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 228009,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static MURAMANA: Item = Item {
    name: "Muramana",
    tier: 4,
    price: 2900,
    stats: &[
        (StatName::AttackDamage, 35),
        (StatName::AbilityHaste, 15),
        (StatName::Mana, 1000),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Muramana,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3042,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static NASHORS_TOOTH: Item = Item {
    name: "Nashor's Tooth",
    tier: 3,
    price: 2900,
    stats: &[
        (StatName::AbilityHaste, 15),
        (StatName::AbilityPower, 80),
        (StatName::AttackSpeed, 50),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::NashorsTooth,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [nashors_tooth_ranged_min, zero],
    melee: [nashors_tooth_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3115,
    identifiers: [
        [&[AbilityPower] as &[_], &[]],
        [&[AbilityPower] as &[_], &[]],
    ],
};

pub const fn nashors_tooth_melee_min(ctx: &Ctx) -> f32 {
    15f32 + 0.15 * ctx.ability_power
}





pub const fn nashors_tooth_ranged_min(ctx: &Ctx) -> f32 {
    15f32 + 0.15 * ctx.ability_power
}







pub static NAVORI_FLICKERBLADE: Item = Item {
    name: "Navori Flickerblade",
    tier: 3,
    price: 2650,
    stats: &[
        (StatName::AttackSpeed, 40),
        (StatName::CritChance, 25),
        (StatName::MoveSpeed, 4),
    ],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::NavoriFlickerblade,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 6675,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static NEEDLESSLY_LARGE_ROD: Item = Item {
    name: "Needlessly Large Rod",
    tier: 1,
    price: 1200,
    stats: &[(StatName::AbilityPower, 65)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::NeedlesslyLargeRod,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1058,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static NEGATRON_CLOAK: Item = Item {
    name: "Negatron Cloak",
    tier: 2,
    price: 850,
    stats: &[(StatName::MagicResist, 45)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::NegatronCloak,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1057,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static NIGHT_HARVESTER: Item = Item {
    name: "Night Harvester",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AbilityHaste, 25),
        (StatName::AbilityPower, 90),
        (StatName::Health, 300),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::NightHarvester,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [night_harvester_ranged_min, zero],
    melee: [night_harvester_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 444636,
    identifiers: [
        [&[AbilityPower, BonusMoveSpeed] as &[_], &[]],
        [&[AbilityPower, BonusMoveSpeed] as &[_], &[]],
    ],
};

pub const fn night_harvester_melee_min(ctx: &Ctx) -> f32 {
    160f32 + 0.4 * ctx.ability_power + 0.4 * ctx.bonus_move_speed
}





pub const fn night_harvester_ranged_min(ctx: &Ctx) -> f32 {
    160f32 + 0.4 * ctx.ability_power + 0.4 * ctx.bonus_move_speed
}







pub static NOONQUIVER: Item = Item {
    name: "Noonquiver",
    tier: 2,
    price: 1300,
    stats: &[(StatName::AttackDamage, 15), (StatName::CritChance, 20)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Noonquiver,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 6670,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static NULL_MAGIC_MANTLE: Item = Item {
    name: "Null-Magic Mantle",
    tier: 1,
    price: 400,
    stats: &[(StatName::MagicResist, 20)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::NullMagicMantle,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1033,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static OBLIVION_ORB: Item = Item {
    name: "Oblivion Orb",
    tier: 2,
    price: 800,
    stats: &[(StatName::AbilityPower, 25)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::OblivionOrb,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3916,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn oblivion_orb_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn oblivion_orb_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static OHMWRECKER_TURRET_ITEM: Item = Item {
    name: "Ohmwrecker (Turret Item)",
    tier: 1,
    price: 0,
    stats: &[(StatName::ArmorPenetration, 30)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::OhmwreckerTurretItem,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1500,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static ORACLE_LENS: Item = Item {
    name: "Oracle Lens",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::OracleLens,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3364,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static OVERCHARGED: Item = Item {
    name: "Overcharged",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[SummonersRift],
    metadata: TypeMetadata {
        kind: ItemId::Overcharged,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1507,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static OVERLORDS_BLOODMAIL: Item = Item {
    name: "Overlord's Bloodmail",
    tier: 3,
    price: 3300,
    stats: &[(StatName::AttackDamage, 30), (StatName::Health, 550)],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::OverlordsBloodmail,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2501,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static PERPLEXITY: Item = Item {
    name: "Perplexity",
    tier: 3,
    price: 2500,
    stats: &[
        (StatName::AbilityPower, 60),
        (StatName::ArmorPenetration, 22),
        (StatName::MagicPenetration, 30),
        (StatName::MoveSpeed, 5),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::Perplexity,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 4015,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static PHAGE: Item = Item {
    name: "Phage",
    tier: 2,
    price: 1100,
    stats: &[(StatName::AttackDamage, 15), (StatName::Health, 200)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Phage,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3044,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static PHANTOM_DANCER: Item = Item {
    name: "Phantom Dancer",
    tier: 3,
    price: 2650,
    stats: &[
        (StatName::AttackSpeed, 65),
        (StatName::CritChance, 25),
        (StatName::MoveSpeed, 10),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::PhantomDancer,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3046,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static PHREAKISH_GUSTO: Item = Item {
    name: "Phreakish Gusto",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::PhreakishGusto,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [phreakish_gusto_ranged_min, zero],
    melee: [phreakish_gusto_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 1510,
    identifiers: [[&[MaxHealth] as &[_], &[]], [&[MaxHealth] as &[_], &[]]],
};

pub const fn phreakish_gusto_melee_min(ctx: &Ctx) -> f32 {
    0.7 * ctx.max_health
}





pub const fn phreakish_gusto_ranged_min(ctx: &Ctx) -> f32 {
    0.7 * ctx.max_health
}







pub static PICKAXE: Item = Item {
    name: "Pickaxe",
    tier: 1,
    price: 875,
    stats: &[(StatName::AttackDamage, 25)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Pickaxe,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1037,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static PLATED_STEELCAPS: Item = Item {
    name: "Plated Steelcaps",
    tier: 2,
    price: 1200,
    stats: &[(StatName::Armor, 25), (StatName::MoveSpeedPercent, 45)],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::PlatedSteelcaps,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3047,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn plated_steelcaps_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn plated_steelcaps_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static PORO_SNAX: Item = Item {
    name: "Poro-Snax",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Aram],
    metadata: TypeMetadata {
        kind: ItemId::PoroSnax,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2052,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static PRISMATIC_ITEM: Item = Item {
    name: "Prismatic Item",
    tier: 1,
    price: 4000,
    stats: &[],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::PrismaticItem,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 220007,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static PRISMATIC_STAT_VOUCHER: Item = Item {
    name: "Prismatic Stat Voucher",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::PrismaticStatVoucher,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 9999,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static PROFANE_HYDRA: Item = Item {
    name: "Profane Hydra",
    tier: 3,
    price: 2850,
    stats: &[
        (StatName::AttackDamage, 55),
        (StatName::AbilityHaste, 10),
        (StatName::Lethality, 18),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ProfaneHydra,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [profane_hydra_ranged_min, zero],
    melee: [profane_hydra_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 6698,
    identifiers: [
        [
            &[AttackDamage, PhysicalMultiplier] as &[_],
            &[PhysicalMultiplier],
        ],
        [
            &[AttackDamage, PhysicalMultiplier] as &[_],
            &[PhysicalMultiplier],
        ],
    ],
};

pub const fn profane_hydra_melee_min(ctx: &Ctx) -> f32 {
    0.8 * ctx.attack_damage
}





pub const fn profane_hydra_ranged_min(ctx: &Ctx) -> f32 {
    0.8 * ctx.attack_damage
}







pub static PROTOPLASM_HARNESS: Item = Item {
    name: "Protoplasm Harness",
    tier: 3,
    price: 2500,
    stats: &[(StatName::AbilityHaste, 20), (StatName::Health, 600)],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ProtoplasmHarness,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2525,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static PROWLERS_CLAW: Item = Item {
    name: "Prowler's Claw",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AttackDamage, 55),
        (StatName::AbilityHaste, 20),
        (StatName::Lethality, 22),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::ProwlersClaw,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [prowlers_claw_ranged_min, zero],
    melee: [prowlers_claw_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 446693,
    identifiers: [
        [
            &[BonusAd, PhysicalMultiplier] as &[_],
            &[PhysicalMultiplier],
        ],
        [
            &[BonusAd, PhysicalMultiplier] as &[_],
            &[PhysicalMultiplier],
        ],
    ],
};

pub const fn prowlers_claw_melee_min(ctx: &Ctx) -> f32 {
    0.0005 * ctx.bonus_ad
}





pub const fn prowlers_claw_ranged_min(ctx: &Ctx) -> f32 {
    0.0005 * ctx.bonus_ad
}







pub static PUPPETEER: Item = Item {
    name: "Puppeteer",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AbilityHaste, 40),
        (StatName::AttackSpeed, 30),
        (StatName::HealAndShieldPower, 15),
        (StatName::BaseManaRegen, 150),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::Puppeteer,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 447123,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static PYROMANCERS_CLOAK: Item = Item {
    name: "Pyromancer's Cloak",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AbilityHaste, 15),
        (StatName::Health, 400),
        (StatName::AdaptiveForce, 85),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::PyromancersCloak,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 447118,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static QUICKSILVER_SASH: Item = Item {
    name: "Quicksilver Sash",
    tier: 2,
    price: 1300,
    stats: &[(StatName::MagicResist, 30)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::QuicksilverSash,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3140,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static RABADONS_DEATHCAP: Item = Item {
    name: "Rabadon's Deathcap",
    tier: 2,
    price: 3500,
    stats: &[(StatName::AbilityPower, 130)],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::RabadonsDeathcap,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3089,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static RADIANT_VIRTUE: Item = Item {
    name: "Radiant Virtue",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::Armor, 35),
        (StatName::Health, 400),
        (StatName::HealAndShieldPower, 12),
        (StatName::MagicResist, 35),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::RadiantVirtue,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 446667,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static RAISE_MORALE: Item = Item {
    name: "Raise Morale",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::RaiseMorale,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3903,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static RANDUINS_OMEN: Item = Item {
    name: "Randuin's Omen",
    tier: 3,
    price: 2700,
    stats: &[(StatName::Armor, 75), (StatName::Health, 350)],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::RanduinsOmen,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3143,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static RAPID_FIRECANNON: Item = Item {
    name: "Rapid Firecannon",
    tier: 3,
    price: 2650,
    stats: &[
        (StatName::AttackSpeed, 35),
        (StatName::CritChance, 25),
        (StatName::MoveSpeed, 4),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::RapidFirecannon,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3094,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static RAVENOUS_HYDRA: Item = Item {
    name: "Ravenous Hydra",
    tier: 3,
    price: 3300,
    stats: &[
        (StatName::AttackDamage, 65),
        (StatName::AbilityHaste, 15),
        (StatName::LifeSteal, 12),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::RavenousHydra,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [ravenous_hydra_ranged_min, zero],
    melee: [ravenous_hydra_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3074,
    identifiers: [
        [
            &[AttackDamage, PhysicalMultiplier] as &[_],
            &[PhysicalMultiplier],
        ],
        [
            &[AttackDamage, PhysicalMultiplier] as &[_],
            &[PhysicalMultiplier],
        ],
    ],
};

pub const fn ravenous_hydra_melee_min(ctx: &Ctx) -> f32 {
    0.8 * ctx.attack_damage
}





pub const fn ravenous_hydra_ranged_min(ctx: &Ctx) -> f32 {
    0.8 * ctx.attack_damage
}







pub static REALITY_FRACTURE: Item = Item {
    name: "Reality Fracture",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AbilityPower, 80),
        (StatName::AttackSpeed, 40),
        (StatName::Health, 300),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::RealityFracture,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [reality_fracture_ranged_min, zero],
    melee: [reality_fracture_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 447102,
    identifiers: [
        [&[AbilityPower, AttackDamage] as &[_], &[]],
        [&[AbilityPower, AttackDamage] as &[_], &[]],
    ],
};

pub const fn reality_fracture_melee_min(ctx: &Ctx) -> f32 {
    6f32 + 0.08 * ctx.ability_power + 0.04 * ctx.attack_damage
}





pub const fn reality_fracture_ranged_min(ctx: &Ctx) -> f32 {
    6f32 + 0.08 * ctx.ability_power + 0.04 * ctx.attack_damage
}







pub static REAPERS_TOLL: Item = Item {
    name: "Reaper's Toll",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AttackSpeed, 50),
        (StatName::MoveSpeed, 5),
        (StatName::AdaptiveForce, 40),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::ReapersToll,
        damage_type: True,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 443090,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static RECTRIX: Item = Item {
    name: "Rectrix",
    tier: 2,
    price: 775,
    stats: &[(StatName::AttackDamage, 15), (StatName::MoveSpeed, 4)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Rectrix,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 6690,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static RECURVE_BOW: Item = Item {
    name: "Recurve Bow",
    tier: 2,
    price: 700,
    stats: &[(StatName::AttackSpeed, 15)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::RecurveBow,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [recurve_bow_ranged_min, zero],
    melee: [recurve_bow_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 1043,
    identifiers: [
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
    ],
};

pub const fn recurve_bow_melee_min(_: &Ctx) -> f32 {
    15f32
}





pub const fn recurve_bow_ranged_min(_: &Ctx) -> f32 {
    15f32
}







pub static REDEMPTION: Item = Item {
    name: "Redemption",
    tier: 3,
    price: 2250,
    stats: &[
        (StatName::AbilityHaste, 15),
        (StatName::AbilityPower, 30),
        (StatName::HealAndShieldPower, 10),
        (StatName::BaseManaRegen, 100),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Redemption,
        damage_type: True,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3107,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static REFILLABLE_POTION: Item = Item {
    name: "Refillable Potion",
    tier: 1,
    price: 150,
    stats: &[],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::RefillablePotion,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2031,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static REGICIDE: Item = Item {
    name: "Regicide",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AttackDamage, 60),
        (StatName::Lethality, 15),
        (StatName::MoveSpeed, 8),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::Regicide,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 447115,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static REINFORCED_ARMOR_TURRET_ITEM: Item = Item {
    name: "Reinforced Armor (Turret Item)",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ReinforcedArmorTurretItem,
        damage_type: True,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 1502,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn reinforced_armor_turret_item_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn reinforced_armor_turret_item_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static REJUVENATION_BEAD: Item = Item {
    name: "Rejuvenation Bead",
    tier: 1,
    price: 300,
    stats: &[(StatName::BaseHealthRegen, 100)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::RejuvenationBead,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1006,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static REVERBERATION: Item = Item {
    name: "Reverberation",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::Armor, 35),
        (StatName::AttackSpeed, 40),
        (StatName::MagicResist, 35),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::Reverberation,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 447114,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static RIFTMAKER: Item = Item {
    name: "Riftmaker",
    tier: 3,
    price: 3100,
    stats: &[
        (StatName::AbilityHaste, 15),
        (StatName::AbilityPower, 70),
        (StatName::Health, 350),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Riftmaker,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 4633,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static RITE_OF_RUIN: Item = Item {
    name: "Rite of Ruin",
    tier: 3,
    price: 2500,
    stats: &[
        (StatName::AbilityHaste, 15),
        (StatName::AbilityPower, 50),
        (StatName::CritChance, 25),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::RiteOfRuin,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3430,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static ROD_OF_AGES: Item = Item {
    name: "Rod of Ages",
    tier: 3,
    price: 2600,
    stats: &[
        (StatName::AbilityPower, 45),
        (StatName::Health, 350),
        (StatName::Mana, 500),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::RodOfAges,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 6657,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static RUBY_CRYSTAL: Item = Item {
    name: "Ruby Crystal",
    tier: 1,
    price: 400,
    stats: &[(StatName::Health, 150)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::RubyCrystal,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1028,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static RUNAANS_HURRICANE: Item = Item {
    name: "Runaan's Hurricane",
    tier: 3,
    price: 2650,
    stats: &[
        (StatName::AttackSpeed, 40),
        (StatName::CritChance, 25),
        (StatName::MoveSpeed, 4),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::RunaansHurricane,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [runaans_hurricane_ranged_min, zero],
    melee: [runaans_hurricane_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3085,
    identifiers: [
        [
            &[AttackDamage, PhysicalMultiplier] as &[_],
            &[PhysicalMultiplier],
        ],
        [
            &[AttackDamage, PhysicalMultiplier] as &[_],
            &[PhysicalMultiplier],
        ],
    ],
};

pub const fn runaans_hurricane_melee_min(ctx: &Ctx) -> f32 {
    0.55 * ctx.attack_damage
}





pub const fn runaans_hurricane_ranged_min(ctx: &Ctx) -> f32 {
    0.55 * ctx.attack_damage
}







pub static RUNECARVER: Item = Item {
    name: "Runecarver",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AbilityHaste, 20),
        (StatName::AbilityPower, 80),
        (StatName::MoveSpeed, 4),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::Runecarver,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 447108,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static RUNIC_COMPASS: Item = Item {
    name: "Runic Compass",
    tier: 2,
    price: 400,
    stats: &[
        (StatName::GoldPer10Seconds, 5),
        (StatName::Health, 100),
        (StatName::BaseHealthRegen, 50),
        (StatName::BaseManaRegen, 50),
    ],
    maps: &[SummonersRift],
    metadata: TypeMetadata {
        kind: ItemId::RunicCompass,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3866,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static RYLAIS_CRYSTAL_SCEPTER: Item = Item {
    name: "Rylai's Crystal Scepter",
    tier: 3,
    price: 2600,
    stats: &[(StatName::AbilityPower, 65), (StatName::Health, 400)],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::RylaisCrystalScepter,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3116,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn rylais_crystal_scepter_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn rylais_crystal_scepter_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static SANGUINE_GIFT: Item = Item {
    name: "Sanguine Gift",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AbilityHaste, 20),
        (StatName::AbilityPower, 80),
        (StatName::HealAndShieldPower, 15),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::SanguineGift,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 443062,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn sanguine_gift_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn sanguine_gift_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static SAPPHIRE_CRYSTAL: Item = Item {
    name: "Sapphire Crystal",
    tier: 1,
    price: 300,
    stats: &[(StatName::Mana, 300)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::SapphireCrystal,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1027,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static SCARECROW_EFFIGY: Item = Item {
    name: "Scarecrow Effigy",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ScarecrowEffigy,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3330,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static SCORCHCLAW_PUP: Item = Item {
    name: "Scorchclaw Pup",
    tier: 1,
    price: 450,
    stats: &[],
    maps: &[SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ScorchclawPup,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1101,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static SCOUTING_AHEAD: Item = Item {
    name: "Scouting Ahead",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[],
    metadata: TypeMetadata {
        kind: ItemId::ScoutingAhead,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 9999,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static SCOUTS_SLINGSHOT: Item = Item {
    name: "Scout's Slingshot",
    tier: 2,
    price: 600,
    stats: &[(StatName::AttackSpeed, 20)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ScoutsSlingshot,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [scouts_slingshot_ranged_min, zero],
    melee: [scouts_slingshot_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3144,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn scouts_slingshot_melee_min(_: &Ctx) -> f32 {
    40f32
}





pub const fn scouts_slingshot_ranged_min(_: &Ctx) -> f32 {
    40f32
}







pub static SEEKERS_ARMGUARD: Item = Item {
    name: "Seeker's Armguard",
    tier: 2,
    price: 1600,
    stats: &[(StatName::AbilityPower, 40), (StatName::Armor, 25)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::SeekersArmguard,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2420,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static SERAPHS_EMBRACE: Item = Item {
    name: "Seraph's Embrace",
    tier: 4,
    price: 2900,
    stats: &[
        (StatName::AbilityHaste, 25),
        (StatName::AbilityPower, 70),
        (StatName::Mana, 1000),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::SeraphsEmbrace,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3040,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static SERPENTS_FANG: Item = Item {
    name: "Serpent's Fang",
    tier: 3,
    price: 2500,
    stats: &[(StatName::AttackDamage, 55), (StatName::Lethality, 15)],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::SerpentsFang,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 6695,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static SERRATED_DIRK: Item = Item {
    name: "Serrated Dirk",
    tier: 2,
    price: 1000,
    stats: &[(StatName::AttackDamage, 20), (StatName::Lethality, 10)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::SerratedDirk,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3134,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static SERYLDAS_GRUDGE: Item = Item {
    name: "Serylda's Grudge",
    tier: 3,
    price: 3000,
    stats: &[
        (StatName::AttackDamage, 45),
        (StatName::AbilityHaste, 15),
        (StatName::ArmorPenetration, 35),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::SeryldasGrudge,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [seryldas_grudge_ranged_min, zero],
    melee: [seryldas_grudge_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 6694,
    identifiers: [[&[MaxHealth] as &[_], &[]], [&[MaxHealth] as &[_], &[]]],
};

pub const fn seryldas_grudge_melee_min(ctx: &Ctx) -> f32 {
    ctx.max_health / 2f32
}





pub const fn seryldas_grudge_ranged_min(ctx: &Ctx) -> f32 {
    ctx.max_health / 2f32
}







pub static SHADOWFLAME: Item = Item {
    name: "Shadowflame",
    tier: 3,
    price: 3200,
    stats: &[
        (StatName::AbilityPower, 110),
        (StatName::MagicPenetration, 15),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Shadowflame,
        damage_type: True,
        attributes: Undefined,
    },
    ranged: [shadowflame_ranged_min, zero],
    melee: [shadowflame_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 4645,
    identifiers: [
        [&[SteelcapsEffect] as &[_], &[]],
        [&[SteelcapsEffect] as &[_], &[]],
    ],
};


pub const fn shadowflame_melee_min(ctx: &Ctx) -> f32 {
    1.2 * ctx.steelcaps_effect
}





pub const fn shadowflame_ranged_min(ctx: &Ctx) -> f32 {
    1.2 * ctx.steelcaps_effect
}







pub static SHATTERED_ARMGUARD: Item = Item {
    name: "Shattered Armguard",
    tier: 2,
    price: 1600,
    stats: &[(StatName::AbilityPower, 40), (StatName::Armor, 25)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ShatteredArmguard,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2421,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static SHEEN: Item = Item {
    name: "Sheen",
    tier: 1,
    price: 900,
    stats: &[(StatName::AbilityHaste, 10)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Sheen,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [sheen_ranged_min, zero],
    melee: [sheen_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3057,
    identifiers: [
        [
            &[AttackDamage, PhysicalMultiplier] as &[_],
            &[PhysicalMultiplier],
        ],
        [
            &[AttackDamage, PhysicalMultiplier] as &[_],
            &[PhysicalMultiplier],
        ],
    ],
};


pub const fn sheen_melee_min(ctx: &Ctx) -> f32 {
    ctx.attack_damage
}





pub const fn sheen_ranged_min(ctx: &Ctx) -> f32 {
    ctx.attack_damage
}







pub static SHIELD_OF_MOLTEN_STONE: Item = Item {
    name: "Shield of Molten Stone",
    tier: 3,
    price: 0,
    stats: &[(StatName::Armor, 100), (StatName::Health, 300)],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::ShieldOfMoltenStone,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 443058,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static SHURELYAS_BATTLESONG: Item = Item {
    name: "Shurelya's Battlesong",
    tier: 3,
    price: 2200,
    stats: &[
        (StatName::AbilityHaste, 15),
        (StatName::AbilityPower, 50),
        (StatName::BaseManaRegen, 125),
        (StatName::MoveSpeed, 4),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ShurelyasBattlesong,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2065,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static SLIGHTLY_MAGICAL_BOOTS: Item = Item {
    name: "Slightly Magical Boots",
    tier: 1,
    price: 0,
    stats: &[(StatName::MoveSpeedPercent, 25)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::SlightlyMagicalBoots,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2422,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static SOLSTICE_SLEIGH: Item = Item {
    name: "Solstice Sleigh",
    tier: 3,
    price: 400,
    stats: &[
        (StatName::GoldPer10Seconds, 9),
        (StatName::Health, 200),
        (StatName::BaseHealthRegen, 75),
        (StatName::BaseManaRegen, 75),
    ],
    maps: &[SummonersRift],
    metadata: TypeMetadata {
        kind: ItemId::SolsticeSleigh,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3876,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static SORCERERS_SHOES: Item = Item {
    name: "Sorcerer's Shoes",
    tier: 2,
    price: 1100,
    stats: &[
        (StatName::MagicPenetration, 12),
        (StatName::MoveSpeedPercent, 45),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::SorcerersShoes,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3020,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static SPEAR_OF_SHOJIN: Item = Item {
    name: "Spear of Shojin",
    tier: 3,
    price: 3100,
    stats: &[(StatName::AttackDamage, 45), (StatName::Health, 450)],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::SpearOfShojin,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3161,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static SPECTRAL_CUTLASS: Item = Item {
    name: "Spectral Cutlass",
    tier: 3,
    price: 2800,
    stats: &[
        (StatName::AttackDamage, 50),
        (StatName::Lethality, 15),
        (StatName::MoveSpeed, 4),
    ],
    maps: &[Arena, Aram, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::SpectralCutlass,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 224004,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static SPECTRES_COWL: Item = Item {
    name: "Spectre's Cowl",
    tier: 2,
    price: 1250,
    stats: &[
        (StatName::Health, 200),
        (StatName::BaseHealthRegen, 100),
        (StatName::MagicResist, 35),
    ],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::SpectresCowl,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3211,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static SPELLSLINGERS_SHOES: Item = Item {
    name: "Spellslinger's Shoes",
    tier: 3,
    price: 1100,
    stats: &[
        (StatName::MagicPenetration, 8),
        (StatName::MagicPenetration, 18),
        (StatName::MoveSpeedPercent, 45),
    ],
    maps: &[SummonersRift],
    metadata: TypeMetadata {
        kind: ItemId::SpellslingersShoes,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3175,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static SPIRIT_VISAGE: Item = Item {
    name: "Spirit Visage",
    tier: 3,
    price: 2700,
    stats: &[
        (StatName::AbilityHaste, 10),
        (StatName::Health, 400),
        (StatName::BaseHealthRegen, 100),
        (StatName::MagicResist, 50),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::SpiritVisage,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3065,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static STAFF_OF_FLOWING_WATER: Item = Item {
    name: "Staff of Flowing Water",
    tier: 3,
    price: 2250,
    stats: &[
        (StatName::AbilityHaste, 10),
        (StatName::AbilityPower, 35),
        (StatName::HealAndShieldPower, 10),
        (StatName::BaseManaRegen, 125),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::StaffOfFlowingWater,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 6616,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static STAT_BONUS_ARAM_MAYHEM: Item = Item {
    name: "Stat Bonus (ARAM: Mayhem)",
    tier: 1,
    price: 750,
    stats: &[],
    maps: &[],
    metadata: TypeMetadata {
        kind: ItemId::StatBonusAramMayhem,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 220000,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static STAT_BONUS_ARENA: Item = Item {
    name: "Stat Bonus (Arena)",
    tier: 1,
    price: 750,
    stats: &[],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::StatBonusArena,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 220000,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static STATIKK_SHIV: Item = Item {
    name: "Statikk Shiv",
    tier: 3,
    price: 3000,
    stats: &[
        (StatName::AttackDamage, 40),
        (StatName::AbilityPower, 45),
        (StatName::AttackSpeed, 30),
        (StatName::MoveSpeed, 4),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::StatikkShiv,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3087,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static STEALTH_WARD: Item = Item {
    name: "Stealth Ward",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::StealthWard,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3340,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static STEEL_SIGIL: Item = Item {
    name: "Steel Sigil",
    tier: 2,
    price: 1100,
    stats: &[(StatName::AttackDamage, 15), (StatName::Armor, 30)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::SteelSigil,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2019,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static STERAKS_GAGE: Item = Item {
    name: "Sterak's Gage",
    tier: 3,
    price: 3200,
    stats: &[(StatName::Health, 400), (StatName::Tenacity, 20)],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::SteraksGage,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3053,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static STORMRAZOR: Item = Item {
    name: "Stormrazor",
    tier: 3,
    price: 3200,
    stats: &[
        (StatName::AttackDamage, 50),
        (StatName::AttackSpeed, 20),
        (StatName::CritChance, 25),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Stormrazor,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3097,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static STORMSURGE: Item = Item {
    name: "Stormsurge",
    tier: 3,
    price: 2800,
    stats: &[
        (StatName::AbilityPower, 90),
        (StatName::MagicPenetration, 15),
        (StatName::MoveSpeed, 6),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Stormsurge,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [stormsurge_ranged_min, zero],
    melee: [stormsurge_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 4646,
    identifiers: [[&[MaxHealth] as &[_], &[]], [&[MaxHealth] as &[_], &[]]],
};


pub const fn stormsurge_melee_min(ctx: &Ctx) -> f32 {
    0.25 * ctx.max_health
}





pub const fn stormsurge_ranged_min(ctx: &Ctx) -> f32 {
    0.25 * ctx.max_health
}







pub static STRIDEBREAKER: Item = Item {
    name: "Stridebreaker",
    tier: 3,
    price: 3300,
    stats: &[
        (StatName::AttackDamage, 40),
        (StatName::AttackSpeed, 25),
        (StatName::Health, 450),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Stridebreaker,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [stridebreaker_ranged_min, zero],
    melee: [stridebreaker_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 6631,
    identifiers: [
        [
            &[AttackDamage, BonusMoveSpeed, PhysicalMultiplier] as &[_],
            &[PhysicalMultiplier],
        ],
        [
            &[AttackDamage, BonusMoveSpeed, PhysicalMultiplier] as &[_],
            &[PhysicalMultiplier],
        ],
    ],
};


pub const fn stridebreaker_melee_min(ctx: &Ctx) -> f32 {
    0.8 * ctx.attack_damage + 0.35 * ctx.bonus_move_speed
}





pub const fn stridebreaker_ranged_min(ctx: &Ctx) -> f32 {
    0.8 * ctx.attack_damage + 0.35 * ctx.bonus_move_speed
}







pub static SUNDERED_SKY: Item = Item {
    name: "Sundered Sky",
    tier: 3,
    price: 3100,
    stats: &[
        (StatName::AttackDamage, 45),
        (StatName::AbilityHaste, 10),
        (StatName::Health, 400),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::SunderedSky,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 6610,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static SUNFIRE_AEGIS: Item = Item {
    name: "Sunfire Aegis",
    tier: 3,
    price: 2700,
    stats: &[
        (StatName::AbilityHaste, 10),
        (StatName::Armor, 50),
        (StatName::Health, 350),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::SunfireAegis,
        damage_type: True,
        attributes: Undefined,
    },
    ranged: [sunfire_aegis_ranged_min, zero],
    melee: [sunfire_aegis_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3068,
    identifiers: [
        [&[BonusHealth, SteelcapsEffect] as &[_], &[]],
        [&[BonusHealth, SteelcapsEffect] as &[_], &[]],
    ],
};

pub const fn sunfire_aegis_melee_min(ctx: &Ctx) -> f32 {
    20f32 + 0.01 * ctx.bonus_health + ctx.steelcaps_effect
}





pub const fn sunfire_aegis_ranged_min(ctx: &Ctx) -> f32 {
    20f32 + 0.01 * ctx.bonus_health + ctx.steelcaps_effect
}







pub static SUPER_MECH_ARMOR: Item = Item {
    name: "Super Mech Armor",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::SuperMechArmor,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [super_mech_armor_ranged_min, zero],
    melee: [super_mech_armor_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 1511,
    identifiers: [[&[MaxHealth] as &[_], &[]], [&[MaxHealth] as &[_], &[]]],
};

pub const fn super_mech_armor_melee_min(ctx: &Ctx) -> f32 {
    0.07 * ctx.max_health
}





pub const fn super_mech_armor_ranged_min(ctx: &Ctx) -> f32 {
    0.07 * ctx.max_health
}







pub static SUPER_MECH_POWER_FIELD: Item = Item {
    name: "Super Mech Power Field",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::SuperMechPowerField,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1512,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static SWIFTMARCH: Item = Item {
    name: "Swiftmarch",
    tier: 3,
    price: 1000,
    stats: &[(StatName::MoveSpeedPercent, 65)],
    maps: &[SummonersRift],
    metadata: TypeMetadata {
        kind: ItemId::Swiftmarch,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3170,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static SWORD_OF_BLOSSOMING_DAWN: Item = Item {
    name: "Sword of Blossoming Dawn",
    tier: 3,
    price: 2500,
    stats: &[
        (StatName::AbilityHaste, 15),
        (StatName::AbilityPower, 45),
        (StatName::Health, 200),
        (StatName::HealAndShieldPower, 12),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::SwordOfBlossomingDawn,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 4011,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static SWORD_OF_THE_DIVINE: Item = Item {
    name: "Sword of the Divine",
    tier: 3,
    price: 0,
    stats: &[(StatName::CritChance, 50), (StatName::AdaptiveForce, 110)],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::SwordOfTheDivine,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [sword_of_the_divine_ranged_min, zero],
    melee: [sword_of_the_divine_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 443060,
    identifiers: [[&[CritChance] as &[_], &[]], [&[CritChance] as &[_], &[]]],
};

pub const fn sword_of_the_divine_melee_min(ctx: &Ctx) -> f32 {
    ctx.crit_chance / 2f32
}





pub const fn sword_of_the_divine_ranged_min(ctx: &Ctx) -> f32 {
    ctx.crit_chance / 2f32
}







pub static TALISMAN_OF_ASCENSION: Item = Item {
    name: "Talisman of Ascension",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AttackDamage, 0),
        (StatName::AbilityHaste, 0),
        (StatName::AbilityPower, 0),
        (StatName::Armor, 0),
        (StatName::ArmorPenetration, 0),
        (StatName::AttackSpeed, 0),
        (StatName::CritChance, 0),
        (StatName::CritDamage, 0),
        (StatName::Health, 0),
        (StatName::BaseHealthRegen, 0),
        (StatName::HealAndShieldPower, 0),
        (StatName::Lethality, 0),
        (StatName::LifeSteal, 0),
        (StatName::Mana, 0),
        (StatName::BaseManaRegen, 0),
        (StatName::MagicPenetration, 0),
        (StatName::MagicPenetration, 0),
        (StatName::MagicResist, 0),
        (StatName::MoveSpeed, 0),
        (StatName::MoveSpeedPercent, 0),
        (StatName::Omnivamp, 0),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::TalismanOfAscension,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 443064,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static TEAR_OF_THE_GODDESS: Item = Item {
    name: "Tear of the Goddess",
    tier: 1,
    price: 400,
    stats: &[(StatName::Mana, 240)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::TearOfTheGoddess,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3070,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static TERMINUS: Item = Item {
    name: "Terminus",
    tier: 3,
    price: 3000,
    stats: &[(StatName::AttackDamage, 30), (StatName::AttackSpeed, 35)],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Terminus,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [terminus_ranged_min, zero],
    melee: [terminus_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3302,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};


pub const fn terminus_melee_min(_: &Ctx) -> f32 {
    30f32
}





pub const fn terminus_ranged_min(_: &Ctx) -> f32 {
    30f32
}







pub static THE_BRUTALIZER: Item = Item {
    name: "The Brutalizer",
    tier: 2,
    price: 1337,
    stats: &[
        (StatName::AttackDamage, 25),
        (StatName::AbilityHaste, 10),
        (StatName::Lethality, 5),
    ],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::TheBrutalizer,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2020,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static THE_COLLECTOR: Item = Item {
    name: "The Collector",
    tier: 3,
    price: 3000,
    stats: &[
        (StatName::AttackDamage, 50),
        (StatName::CritChance, 25),
        (StatName::Lethality, 10),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::TheCollector,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [the_collector_ranged_min, zero],
    melee: [the_collector_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 6676,
    identifiers: [[&[MaxHealth] as &[_], &[]], [&[MaxHealth] as &[_], &[]]],
};

pub const fn the_collector_melee_min(ctx: &Ctx) -> f32 {
    0.05 * ctx.max_health
}





pub const fn the_collector_ranged_min(ctx: &Ctx) -> f32 {
    0.05 * ctx.max_health
}







pub static THE_GOLDEN_SPATULA: Item = Item {
    name: "The Golden Spatula",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AttackDamage, 90),
        (StatName::AbilityHaste, 20),
        (StatName::AbilityPower, 125),
        (StatName::Armor, 40),
        (StatName::AttackSpeed, 60),
        (StatName::CritChance, 25),
        (StatName::Health, 350),
        (StatName::BaseHealthRegen, 200),
        (StatName::Mana, 350),
        (StatName::BaseManaRegen, 200),
        (StatName::MagicResist, 40),
        (StatName::MoveSpeed, 10),
        (StatName::Omnivamp, 15),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::TheGoldenSpatula,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 224403,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static THORNMAIL: Item = Item {
    name: "Thornmail",
    tier: 3,
    price: 2450,
    stats: &[(StatName::Armor, 75), (StatName::Health, 150)],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Thornmail,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [thornmail_ranged_min, zero],
    melee: [thornmail_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3075,
    identifiers: [[&[BonusArmor] as &[_], &[]], [&[BonusArmor] as &[_], &[]]],
};


pub const fn thornmail_melee_min(ctx: &Ctx) -> f32 {
    20f32 + 0.1 * ctx.bonus_armor
}





pub const fn thornmail_ranged_min(ctx: &Ctx) -> f32 {
    20f32 + 0.1 * ctx.bonus_armor
}







pub static TIAMAT: Item = Item {
    name: "Tiamat",
    tier: 2,
    price: 1200,
    stats: &[(StatName::AttackDamage, 20)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Tiamat,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [tiamat_ranged_min, zero],
    melee: [tiamat_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3077,
    identifiers: [
        [
            &[AttackDamage, PhysicalMultiplier] as &[_],
            &[PhysicalMultiplier],
        ],
        [
            &[AttackDamage, PhysicalMultiplier] as &[_],
            &[PhysicalMultiplier],
        ],
    ],
};


pub const fn tiamat_melee_min(ctx: &Ctx) -> f32 {
    0.75 * ctx.attack_damage
}





pub const fn tiamat_ranged_min(ctx: &Ctx) -> f32 {
    0.75 * ctx.attack_damage
}







pub static TITANIC_HYDRA: Item = Item {
    name: "Titanic Hydra",
    tier: 3,
    price: 3300,
    stats: &[(StatName::AttackDamage, 40), (StatName::Health, 600)],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::TitanicHydra,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3748,
    identifiers: [
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
        [&[PhysicalMultiplier] as &[_], &[PhysicalMultiplier]],
    ],
};












pub static TOTAL_BISCUIT_OF_EVERLASTING_WILL: Item = Item {
    name: "Total Biscuit of Everlasting Will",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::TotalBiscuitOfEverlastingWill,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2010,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static TOWER_POWER_UP: Item = Item {
    name: "Tower Power-Up",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Aram],
    metadata: TypeMetadata {
        kind: ItemId::TowerPowerUp,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 999999,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static TRINITY_FORCE: Item = Item {
    name: "Trinity Force",
    tier: 3,
    price: 3333,
    stats: &[
        (StatName::AttackDamage, 36),
        (StatName::AbilityHaste, 15),
        (StatName::AttackSpeed, 30),
        (StatName::Health, 333),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::TrinityForce,
        damage_type: Physical,
        attributes: Undefined,
    },
    ranged: [trinity_force_ranged_min, zero],
    melee: [trinity_force_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3078,
    identifiers: [
        [
            &[AttackDamage, PhysicalMultiplier] as &[_],
            &[PhysicalMultiplier],
        ],
        [
            &[AttackDamage, PhysicalMultiplier] as &[_],
            &[PhysicalMultiplier],
        ],
    ],
};

pub const fn trinity_force_melee_min(ctx: &Ctx) -> f32 {
    2f32 * ctx.attack_damage
}





pub const fn trinity_force_ranged_min(ctx: &Ctx) -> f32 {
    2f32 * ctx.attack_damage
}







pub static TUNNELER: Item = Item {
    name: "Tunneler",
    tier: 2,
    price: 1150,
    stats: &[(StatName::AttackDamage, 15), (StatName::Health, 250)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Tunneler,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2021,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static TURBO_CHEMTANK: Item = Item {
    name: "Turbo Chemtank",
    tier: 3,
    price: 0,
    stats: &[(StatName::Health, 600), (StatName::AdaptiveForce, 80)],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::TurboChemtank,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 443079,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static TURRET_PLATING: Item = Item {
    name: "Turret Plating",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::TurretPlating,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1515,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static TWILIGHTS_EDGE: Item = Item {
    name: "Twilight's Edge",
    tier: 3,
    price: 0,
    stats: &[(StatName::AttackDamage, 70), (StatName::AbilityPower, 100)],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::TwilightsEdge,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 447121,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static TWIN_MASK: Item = Item {
    name: "Twin Mask",
    tier: 3,
    price: 0,
    stats: &[],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::TwinMask,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 443080,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static UMBRAL_GLAIVE: Item = Item {
    name: "Umbral Glaive",
    tier: 3,
    price: 2800,
    stats: &[
        (StatName::AttackDamage, 60),
        (StatName::AbilityHaste, 15),
        (StatName::Lethality, 18),
    ],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::UmbralGlaive,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3179,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static UNENDING_DESPAIR: Item = Item {
    name: "Unending Despair",
    tier: 3,
    price: 2800,
    stats: &[
        (StatName::AbilityHaste, 15),
        (StatName::Armor, 50),
        (StatName::Health, 400),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::UnendingDespair,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [unending_despair_ranged_min, zero],
    melee: [unending_despair_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 2502,
    identifiers: [[&[BonusHealth] as &[_], &[]], [&[BonusHealth] as &[_], &[]]],
};

pub const fn unending_despair_melee_min(ctx: &Ctx) -> f32 {
    0.03 * ctx.bonus_health
}





pub const fn unending_despair_ranged_min(ctx: &Ctx) -> f32 {
    0.03 * ctx.bonus_health
}







pub static VAMPIRIC_SCEPTER: Item = Item {
    name: "Vampiric Scepter",
    tier: 2,
    price: 900,
    stats: &[(StatName::AttackDamage, 15), (StatName::LifeSteal, 7)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::VampiricScepter,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1053,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static VEIGARS_TALISMAN_OF_ASCENSION: Item = Item {
    name: "Veigar's Talisman of Ascension",
    tier: 3,
    price: 900,
    stats: &[],
    maps: &[],
    metadata: TypeMetadata {
        kind: ItemId::VeigarsTalismanOfAscension,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 999999,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static VERDANT_BARRIER: Item = Item {
    name: "Verdant Barrier",
    tier: 2,
    price: 1600,
    stats: &[(StatName::AbilityPower, 40), (StatName::MagicResist, 25)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::VerdantBarrier,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 4632,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn verdant_barrier_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn verdant_barrier_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static VOID_IMMOLATION: Item = Item {
    name: "Void Immolation",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AbilityHaste, 25),
        (StatName::Armor, 100),
        (StatName::Health, 1000),
        (StatName::BaseHealthRegen, 200),
        (StatName::MagicResist, 80),
    ],
    maps: &[],
    metadata: TypeMetadata {
        kind: ItemId::VoidImmolation,
        damage_type: True,
        attributes: Undefined,
    },
    ranged: [void_immolation_ranged_min, zero],
    melee: [void_immolation_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 223069,
    identifiers: [
        [&[MaxHealth, SteelcapsEffect] as &[_], &[]],
        [&[MaxHealth, SteelcapsEffect] as &[_], &[]],
    ],
};

pub const fn void_immolation_melee_min(ctx: &Ctx) -> f32 {
    20f32 + 0.015 * ctx.max_health + ctx.steelcaps_effect
}





pub const fn void_immolation_ranged_min(ctx: &Ctx) -> f32 {
    20f32 + 0.015 * ctx.max_health + ctx.steelcaps_effect
}







pub static VOID_STAFF: Item = Item {
    name: "Void Staff",
    tier: 3,
    price: 3000,
    stats: &[
        (StatName::AbilityPower, 95),
        (StatName::MagicPenetration, 40),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::VoidStaff,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3135,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static VOLTAIC_CYCLOSWORD: Item = Item {
    name: "Voltaic Cyclosword",
    tier: 3,
    price: 3000,
    stats: &[
        (StatName::AttackDamage, 55),
        (StatName::AbilityHaste, 10),
        (StatName::Lethality, 10),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::VoltaicCyclosword,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 6699,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static WARDENS_EYE: Item = Item {
    name: "Warden's Eye",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::WardensEye,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 1503,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static WARDENS_MAIL: Item = Item {
    name: "Warden's Mail",
    tier: 2,
    price: 1000,
    stats: &[(StatName::Armor, 40)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::WardensMail,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3082,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn wardens_mail_melee_min(_: &Ctx) -> f32 {
    0f32
}





pub const fn wardens_mail_ranged_min(_: &Ctx) -> f32 {
    0f32
}







pub static WARMOGS_ARMOR: Item = Item {
    name: "Warmog's Armor",
    tier: 3,
    price: 3100,
    stats: &[(StatName::Health, 1000), (StatName::BaseHealthRegen, 100)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::WarmogsArmor,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3083,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static WHISPERING_CIRCLET: Item = Item {
    name: "Whispering Circlet",
    tier: 3,
    price: 2250,
    stats: &[
        (StatName::Health, 200),
        (StatName::HealAndShieldPower, 8),
        (StatName::Mana, 300),
        (StatName::BaseManaRegen, 75),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::WhisperingCirclet,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 2526,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static WINGED_MOONPLATE: Item = Item {
    name: "Winged Moonplate",
    tier: 2,
    price: 800,
    stats: &[(StatName::Health, 200), (StatName::MoveSpeed, 4)],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::WingedMoonplate,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3066,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static WINTERS_APPROACH: Item = Item {
    name: "Winter's Approach",
    tier: 3,
    price: 2400,
    stats: &[
        (StatName::AbilityHaste, 15),
        (StatName::Health, 550),
        (StatName::Mana, 500),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::WintersApproach,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3119,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static WITS_END: Item = Item {
    name: "Wit's End",
    tier: 3,
    price: 2800,
    stats: &[
        (StatName::AttackSpeed, 50),
        (StatName::MagicResist, 45),
        (StatName::Tenacity, 20),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::WitsEnd,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [wits_end_ranged_min, zero],
    melee: [wits_end_melee_min, zero],
    deals_damage: [true, false, true, false],
    purchasable: false,
    riot_id: 3091,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};

pub const fn wits_end_melee_min(_: &Ctx) -> f32 {
    45f32
}





pub const fn wits_end_ranged_min(_: &Ctx) -> f32 {
    45f32
}







pub static WOOGLETS_WITCHCAP: Item = Item {
    name: "Wooglet's Witchcap",
    tier: 3,
    price: 0,
    stats: &[
        (StatName::AbilityHaste, 20),
        (StatName::AbilityPower, 300),
        (StatName::Armor, 50),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::WoogletsWitchcap,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 228002,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static WORDLESS_PROMISE: Item = Item {
    name: "Wordless Promise",
    tier: 3,
    price: 2500,
    stats: &[
        (StatName::AbilityHaste, 25),
        (StatName::AbilityPower, 50),
        (StatName::HealAndShieldPower, 25),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::WordlessPromise,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 4016,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static WORLD_ATLAS: Item = Item {
    name: "World Atlas",
    tier: 1,
    price: 400,
    stats: &[
        (StatName::GoldPer10Seconds, 3),
        (StatName::Health, 30),
        (StatName::BaseHealthRegen, 25),
        (StatName::BaseManaRegen, 25),
    ],
    maps: &[SummonersRift],
    metadata: TypeMetadata {
        kind: ItemId::WorldAtlas,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3865,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static YOUMUUS_GHOSTBLADE: Item = Item {
    name: "Youmuu's Ghostblade",
    tier: 3,
    price: 2800,
    stats: &[
        (StatName::AttackDamage, 55),
        (StatName::Lethality, 18),
        (StatName::MoveSpeed, 4),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::YoumuusGhostblade,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3142,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static YOUR_CUT: Item = Item {
    name: "Your Cut",
    tier: 1,
    price: 0,
    stats: &[],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::YourCut,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3400,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static YUN_TAL_WILDARROWS: Item = Item {
    name: "Yun Tal Wildarrows",
    tier: 3,
    price: 3100,
    stats: &[
        (StatName::AttackDamage, 50),
        (StatName::AttackSpeed, 40),
        (StatName::CritChance, 0),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::YunTalWildarrows,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3032,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static ZAZ_ZAKS_REALMSPIKE: Item = Item {
    name: "Zaz'Zak's Realmspike",
    tier: 3,
    price: 400,
    stats: &[
        (StatName::GoldPer10Seconds, 9),
        (StatName::Health, 200),
        (StatName::BaseHealthRegen, 75),
        (StatName::BaseManaRegen, 75),
    ],
    maps: &[SummonersRift],
    metadata: TypeMetadata {
        kind: ItemId::ZazZaksRealmspike,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3871,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static ZEAL: Item = Item {
    name: "Zeal",
    tier: 2,
    price: 1200,
    stats: &[
        (StatName::AttackSpeed, 15),
        (StatName::CritChance, 15),
        (StatName::MoveSpeed, 4),
    ],
    maps: &[Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::Zeal,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3086,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static ZEKES_CONVERGENCE: Item = Item {
    name: "Zeke's Convergence",
    tier: 3,
    price: 2200,
    stats: &[
        (StatName::AbilityHaste, 10),
        (StatName::Armor, 25),
        (StatName::Health, 300),
        (StatName::MagicResist, 25),
    ],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ZekesConvergence,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3050,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static ZEPHYR: Item = Item {
    name: "Zephyr",
    tier: 3,
    price: 2500,
    stats: &[
        (StatName::AbilityHaste, 30),
        (StatName::AttackSpeed, 50),
        (StatName::MoveSpeed, 10),
        (StatName::Tenacity, 20),
    ],
    maps: &[Arena],
    metadata: TypeMetadata {
        kind: ItemId::Zephyr,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3172,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};












pub static ZHONYAS_HOURGLASS: Item = Item {
    name: "Zhonya's Hourglass",
    tier: 3,
    price: 3250,
    stats: &[(StatName::AbilityPower, 105), (StatName::Armor, 50)],
    maps: &[Arena, Aram, SummonersRift, NexusBlitz],
    metadata: TypeMetadata {
        kind: ItemId::ZhonyasHourglass,
        damage_type: Unspecified,
        attributes: Undefined,
    },
    ranged: [zero, zero],
    melee: [zero, zero],
    deals_damage: [false, false, false, false],
    purchasable: false,
    riot_id: 3157,
    identifiers: [[&[] as &[_], &[]], [&[] as &[_], &[]]],
};










#[derive(
    Clone,
    Copy,
    Debug,
    Decode,
    Deserialize,
    Eq,
    Encode,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[repr(u16)]
pub enum ItemId {
    AbyssalMask,
    Actualizer,
    AetherWisp,
    AmplifyingTome,
    AnathemasChains,
    AntiTowerSocks,
    AnvilVoucher,
    ArcaneSweeperTrinket,
    ArchangelsStaff,
    ArdentCenser,
    ArmoredAdvance,
    AtmasReckoning,
    AxiomArc,
    BFSword,
    BamisCinder,
    BandleglassMirror,
    Bandlepipes,
    BansheesVeil,
    BaseTurretReinforcedArmorTurretItem,
    Bastionbreaker,
    BerserkersGreaves,
    BlackCleaver,
    BlackHoleGauntlet,
    BlackSpear,
    BlackfireTorch,
    BladeOfTheRuinedKing,
    BlastingWand,
    BlightingJewel,
    BloodlettersCurse,
    Bloodsong,
    Bloodthirster,
    Boots,
    BootsOfSwiftness,
    BountyOfWorlds,
    BrambleVest,
    BraveryVoucher,
    CappaJuice,
    CatalystOfAeons,
    CaulfieldsWarhammer,
    CelestialOpposition,
    ChainVest,
    ChainlacedCrushers,
    ChempunkChainsword,
    CloakOfAgility,
    CloakOfStarryNight,
    ClothArmor,
    ControlWard,
    CosmicDrive,
    CrimsonLucidity,
    CrownOfTheShatteredQueen,
    Cruelty,
    Cryptbloom,
    CrystallineBracer,
    CrystallineOvergrowth,
    Cull,
    Dagger,
    DarkSeal,
    DarksteelTalons,
    Dawncore,
    DeadMansPlate,
    DeathsDance,
    DeathsDaughter,
    Decapitator,
    DemonKingsCrown,
    DemonicEmbrace,
    DetonationOrb,
    DiademOfSongs,
    DiamondTippedSpear,
    DivineSunderer,
    DoransBlade,
    DoransBow,
    DoransHelm,
    DoransRing,
    DoransShield,
    Dragonheart,
    DreamMaker,
    DuskAndDawn,
    DuskbladeOfDraktharr,
    EchoesOfHelia,
    Eclipse,
    EdgeOfNight,
    EleisasMiracle,
    ElixirOfAvarice,
    ElixirOfForce,
    ElixirOfIron,
    ElixirOfSkill,
    ElixirOfSorcery,
    ElixirOfWrath,
    EmpyreanPromise,
    EndlessHunger,
    EnhancedLuckyDice,
    EssenceReaver,
    Everfrost,
    ExecutionersCalling,
    ExperimentalHexplate,
    EyeOfTheHerald,
    FaerieCharm,
    FarsightAlteration,
    FatedAshes,
    FiendhunterBolts,
    FiendishCodex,
    Fimbulwinter,
    FireAtWill,
    Flesheater,
    ForbiddenIdol,
    ForceOfEntropy,
    ForceOfNature,
    FortificationAram,
    FrozenHeart,
    Fulmination,
    Galeforce,
    GamblersBlade,
    GargoyleStoneplate,
    Ghostcrawlers,
    GiantsBelt,
    GlacialBuckler,
    GlowingMote,
    GluttonousGreaves,
    GoldStatAnvilVoucher,
    Goredrinker,
    GuardianAngel,
    GuardiansAmulet,
    GuardiansBlade,
    GuardiansDirk,
    GuardiansHammer,
    GuardiansHorn,
    GuardiansOrb,
    GuardiansShroud,
    GuinsoosRageblade,
    GunmetalGreaves,
    Gusto,
    GustwalkerHatchling,
    Hamstringer,
    HauntingGuise,
    HealthPotion,
    HearthboundAxe,
    Heartsteel,
    HellfireHatchet,
    HemomancersHelm,
    HexboltCompanion,
    Hexdrinker,
    HexopticsC44,
    HextechAlternator,
    HextechGunblade,
    HextechRocketbelt,
    HollowRadiance,
    HorizonFocus,
    Hubris,
    Hullbreaker,
    IcebornGauntlet,
    ImmortalPath,
    ImmortalShieldbow,
    ImperialMandate,
    InfinityEdge,
    InnervatingLocket,
    IonianBootsOfLucidity,
    JakShoTheProtean,
    JarvanIs,
    JuiceOfHaste,
    JuiceOfPower,
    JuiceOfVitality,
    KaenicRookern,
    Kindlegem,
    KinkouJitte,
    KnightsVow,
    KrakenSlayer,
    LastWhisper,
    LegendaryAssassinItem,
    LegendaryFighterItem,
    LegendaryMageItem,
    LegendaryMarksmanItem,
    LegendarySupportItem,
    LegendaryTankItem,
    LiandrysTorment,
    LichBane,
    Lifeline,
    LightningRod,
    LocketOfTheIronSolari,
    LongSword,
    LordDominiksRegards,
    LostChapter,
    LuckyDice,
    LudensEcho,
    Malignance,
    Manamune,
    MawOfMalmortius,
    MejaisSoulstealer,
    MercurialScimitar,
    MercurysTreads,
    MikaelsBlessing,
    MirageBlade,
    MoonflairSpellblade,
    MoonstoneRenewer,
    Morellonomicon,
    MortalReminder,
    MosstomperSeedling,
    Multitool,
    Muramana,
    NashorsTooth,
    NavoriFlickerblade,
    NeedlesslyLargeRod,
    NegatronCloak,
    NightHarvester,
    Noonquiver,
    NullMagicMantle,
    OblivionOrb,
    OhmwreckerTurretItem,
    OracleLens,
    Overcharged,
    OverlordsBloodmail,
    Perplexity,
    Phage,
    PhantomDancer,
    PhreakishGusto,
    Pickaxe,
    PlatedSteelcaps,
    PoroSnax,
    PrismaticItem,
    PrismaticStatVoucher,
    ProfaneHydra,
    ProtoplasmHarness,
    ProwlersClaw,
    Puppeteer,
    PyromancersCloak,
    QuicksilverSash,
    RabadonsDeathcap,
    RadiantVirtue,
    RaiseMorale,
    RanduinsOmen,
    RapidFirecannon,
    RavenousHydra,
    RealityFracture,
    ReapersToll,
    Rectrix,
    RecurveBow,
    Redemption,
    RefillablePotion,
    Regicide,
    ReinforcedArmorTurretItem,
    RejuvenationBead,
    Reverberation,
    Riftmaker,
    RiteOfRuin,
    RodOfAges,
    RubyCrystal,
    RunaansHurricane,
    Runecarver,
    RunicCompass,
    RylaisCrystalScepter,
    SanguineGift,
    SapphireCrystal,
    ScarecrowEffigy,
    ScorchclawPup,
    ScoutingAhead,
    ScoutsSlingshot,
    SeekersArmguard,
    SeraphsEmbrace,
    SerpentsFang,
    SerratedDirk,
    SeryldasGrudge,
    Shadowflame,
    ShatteredArmguard,
    Sheen,
    ShieldOfMoltenStone,
    ShurelyasBattlesong,
    SlightlyMagicalBoots,
    SolsticeSleigh,
    SorcerersShoes,
    SpearOfShojin,
    SpectralCutlass,
    SpectresCowl,
    SpellslingersShoes,
    SpiritVisage,
    StaffOfFlowingWater,
    StatBonusAramMayhem,
    StatBonusArena,
    StatikkShiv,
    StealthWard,
    SteelSigil,
    SteraksGage,
    Stormrazor,
    Stormsurge,
    Stridebreaker,
    SunderedSky,
    SunfireAegis,
    SuperMechArmor,
    SuperMechPowerField,
    Swiftmarch,
    SwordOfBlossomingDawn,
    SwordOfTheDivine,
    TalismanOfAscension,
    TearOfTheGoddess,
    Terminus,
    TheBrutalizer,
    TheCollector,
    TheGoldenSpatula,
    Thornmail,
    Tiamat,
    TitanicHydra,
    TotalBiscuitOfEverlastingWill,
    TowerPowerUp,
    TrinityForce,
    Tunneler,
    TurboChemtank,
    TurretPlating,
    TwilightsEdge,
    TwinMask,
    UmbralGlaive,
    UnendingDespair,
    VampiricScepter,
    VeigarsTalismanOfAscension,
    VerdantBarrier,
    VoidImmolation,
    VoidStaff,
    VoltaicCyclosword,
    WardensEye,
    WardensMail,
    WarmogsArmor,
    WhisperingCirclet,
    WingedMoonplate,
    WintersApproach,
    WitsEnd,
    WoogletsWitchcap,
    WordlessPromise,
    WorldAtlas,
    YoumuusGhostblade,
    YourCut,
    YunTalWildarrows,
    ZazZaksRealmspike,
    Zeal,
    ZekesConvergence,
    Zephyr,
    ZhonyasHourglass,
}

impl ItemId {
    pub const VARIANTS: usize = 333;
    pub const fn debug(&self) -> &'static str {
        match self {
            Self::AbyssalMask => "AbyssalMask",
            Self::Actualizer => "Actualizer",
            Self::AetherWisp => "AetherWisp",
            Self::AmplifyingTome => "AmplifyingTome",
            Self::AnathemasChains => "AnathemasChains",
            Self::AntiTowerSocks => "AntiTowerSocks",
            Self::AnvilVoucher => "AnvilVoucher",
            Self::ArcaneSweeperTrinket => "ArcaneSweeperTrinket",
            Self::ArchangelsStaff => "ArchangelsStaff",
            Self::ArdentCenser => "ArdentCenser",
            Self::ArmoredAdvance => "ArmoredAdvance",
            Self::AtmasReckoning => "AtmasReckoning",
            Self::AxiomArc => "AxiomArc",
            Self::BFSword => "BFSword",
            Self::BamisCinder => "BamisCinder",
            Self::BandleglassMirror => "BandleglassMirror",
            Self::Bandlepipes => "Bandlepipes",
            Self::BansheesVeil => "BansheesVeil",
            Self::BaseTurretReinforcedArmorTurretItem => {
                "BaseTurretReinforcedArmorTurretItem"
            }
            Self::Bastionbreaker => "Bastionbreaker",
            Self::BerserkersGreaves => "BerserkersGreaves",
            Self::BlackCleaver => "BlackCleaver",
            Self::BlackHoleGauntlet => "BlackHoleGauntlet",
            Self::BlackSpear => "BlackSpear",
            Self::BlackfireTorch => "BlackfireTorch",
            Self::BladeOfTheRuinedKing => "BladeOfTheRuinedKing",
            Self::BlastingWand => "BlastingWand",
            Self::BlightingJewel => "BlightingJewel",
            Self::BloodlettersCurse => "BloodlettersCurse",
            Self::Bloodsong => "Bloodsong",
            Self::Bloodthirster => "Bloodthirster",
            Self::Boots => "Boots",
            Self::BootsOfSwiftness => "BootsOfSwiftness",
            Self::BountyOfWorlds => "BountyOfWorlds",
            Self::BrambleVest => "BrambleVest",
            Self::BraveryVoucher => "BraveryVoucher",
            Self::CappaJuice => "CappaJuice",
            Self::CatalystOfAeons => "CatalystOfAeons",
            Self::CaulfieldsWarhammer => "CaulfieldsWarhammer",
            Self::CelestialOpposition => "CelestialOpposition",
            Self::ChainVest => "ChainVest",
            Self::ChainlacedCrushers => "ChainlacedCrushers",
            Self::ChempunkChainsword => "ChempunkChainsword",
            Self::CloakOfAgility => "CloakOfAgility",
            Self::CloakOfStarryNight => "CloakOfStarryNight",
            Self::ClothArmor => "ClothArmor",
            Self::ControlWard => "ControlWard",
            Self::CosmicDrive => "CosmicDrive",
            Self::CrimsonLucidity => "CrimsonLucidity",
            Self::CrownOfTheShatteredQueen => "CrownOfTheShatteredQueen",
            Self::Cruelty => "Cruelty",
            Self::Cryptbloom => "Cryptbloom",
            Self::CrystallineBracer => "CrystallineBracer",
            Self::CrystallineOvergrowth => "CrystallineOvergrowth",
            Self::Cull => "Cull",
            Self::Dagger => "Dagger",
            Self::DarkSeal => "DarkSeal",
            Self::DarksteelTalons => "DarksteelTalons",
            Self::Dawncore => "Dawncore",
            Self::DeadMansPlate => "DeadMansPlate",
            Self::DeathsDance => "DeathsDance",
            Self::DeathsDaughter => "DeathsDaughter",
            Self::Decapitator => "Decapitator",
            Self::DemonKingsCrown => "DemonKingsCrown",
            Self::DemonicEmbrace => "DemonicEmbrace",
            Self::DetonationOrb => "DetonationOrb",
            Self::DiademOfSongs => "DiademOfSongs",
            Self::DiamondTippedSpear => "DiamondTippedSpear",
            Self::DivineSunderer => "DivineSunderer",
            Self::DoransBlade => "DoransBlade",
            Self::DoransBow => "DoransBow",
            Self::DoransHelm => "DoransHelm",
            Self::DoransRing => "DoransRing",
            Self::DoransShield => "DoransShield",
            Self::Dragonheart => "Dragonheart",
            Self::DreamMaker => "DreamMaker",
            Self::DuskAndDawn => "DuskAndDawn",
            Self::DuskbladeOfDraktharr => "DuskbladeOfDraktharr",
            Self::EchoesOfHelia => "EchoesOfHelia",
            Self::Eclipse => "Eclipse",
            Self::EdgeOfNight => "EdgeOfNight",
            Self::EleisasMiracle => "EleisasMiracle",
            Self::ElixirOfAvarice => "ElixirOfAvarice",
            Self::ElixirOfForce => "ElixirOfForce",
            Self::ElixirOfIron => "ElixirOfIron",
            Self::ElixirOfSkill => "ElixirOfSkill",
            Self::ElixirOfSorcery => "ElixirOfSorcery",
            Self::ElixirOfWrath => "ElixirOfWrath",
            Self::EmpyreanPromise => "EmpyreanPromise",
            Self::EndlessHunger => "EndlessHunger",
            Self::EnhancedLuckyDice => "EnhancedLuckyDice",
            Self::EssenceReaver => "EssenceReaver",
            Self::Everfrost => "Everfrost",
            Self::ExecutionersCalling => "ExecutionersCalling",
            Self::ExperimentalHexplate => "ExperimentalHexplate",
            Self::EyeOfTheHerald => "EyeOfTheHerald",
            Self::FaerieCharm => "FaerieCharm",
            Self::FarsightAlteration => "FarsightAlteration",
            Self::FatedAshes => "FatedAshes",
            Self::FiendhunterBolts => "FiendhunterBolts",
            Self::FiendishCodex => "FiendishCodex",
            Self::Fimbulwinter => "Fimbulwinter",
            Self::FireAtWill => "FireAtWill",
            Self::Flesheater => "Flesheater",
            Self::ForbiddenIdol => "ForbiddenIdol",
            Self::ForceOfEntropy => "ForceOfEntropy",
            Self::ForceOfNature => "ForceOfNature",
            Self::FortificationAram => "FortificationAram",
            Self::FrozenHeart => "FrozenHeart",
            Self::Fulmination => "Fulmination",
            Self::Galeforce => "Galeforce",
            Self::GamblersBlade => "GamblersBlade",
            Self::GargoyleStoneplate => "GargoyleStoneplate",
            Self::Ghostcrawlers => "Ghostcrawlers",
            Self::GiantsBelt => "GiantsBelt",
            Self::GlacialBuckler => "GlacialBuckler",
            Self::GlowingMote => "GlowingMote",
            Self::GluttonousGreaves => "GluttonousGreaves",
            Self::GoldStatAnvilVoucher => "GoldStatAnvilVoucher",
            Self::Goredrinker => "Goredrinker",
            Self::GuardianAngel => "GuardianAngel",
            Self::GuardiansAmulet => "GuardiansAmulet",
            Self::GuardiansBlade => "GuardiansBlade",
            Self::GuardiansDirk => "GuardiansDirk",
            Self::GuardiansHammer => "GuardiansHammer",
            Self::GuardiansHorn => "GuardiansHorn",
            Self::GuardiansOrb => "GuardiansOrb",
            Self::GuardiansShroud => "GuardiansShroud",
            Self::GuinsoosRageblade => "GuinsoosRageblade",
            Self::GunmetalGreaves => "GunmetalGreaves",
            Self::Gusto => "Gusto",
            Self::GustwalkerHatchling => "GustwalkerHatchling",
            Self::Hamstringer => "Hamstringer",
            Self::HauntingGuise => "HauntingGuise",
            Self::HealthPotion => "HealthPotion",
            Self::HearthboundAxe => "HearthboundAxe",
            Self::Heartsteel => "Heartsteel",
            Self::HellfireHatchet => "HellfireHatchet",
            Self::HemomancersHelm => "HemomancersHelm",
            Self::HexboltCompanion => "HexboltCompanion",
            Self::Hexdrinker => "Hexdrinker",
            Self::HexopticsC44 => "HexopticsC44",
            Self::HextechAlternator => "HextechAlternator",
            Self::HextechGunblade => "HextechGunblade",
            Self::HextechRocketbelt => "HextechRocketbelt",
            Self::HollowRadiance => "HollowRadiance",
            Self::HorizonFocus => "HorizonFocus",
            Self::Hubris => "Hubris",
            Self::Hullbreaker => "Hullbreaker",
            Self::IcebornGauntlet => "IcebornGauntlet",
            Self::ImmortalPath => "ImmortalPath",
            Self::ImmortalShieldbow => "ImmortalShieldbow",
            Self::ImperialMandate => "ImperialMandate",
            Self::InfinityEdge => "InfinityEdge",
            Self::InnervatingLocket => "InnervatingLocket",
            Self::IonianBootsOfLucidity => "IonianBootsOfLucidity",
            Self::JakShoTheProtean => "JakShoTheProtean",
            Self::JarvanIs => "JarvanIs",
            Self::JuiceOfHaste => "JuiceOfHaste",
            Self::JuiceOfPower => "JuiceOfPower",
            Self::JuiceOfVitality => "JuiceOfVitality",
            Self::KaenicRookern => "KaenicRookern",
            Self::Kindlegem => "Kindlegem",
            Self::KinkouJitte => "KinkouJitte",
            Self::KnightsVow => "KnightsVow",
            Self::KrakenSlayer => "KrakenSlayer",
            Self::LastWhisper => "LastWhisper",
            Self::LegendaryAssassinItem => "LegendaryAssassinItem",
            Self::LegendaryFighterItem => "LegendaryFighterItem",
            Self::LegendaryMageItem => "LegendaryMageItem",
            Self::LegendaryMarksmanItem => "LegendaryMarksmanItem",
            Self::LegendarySupportItem => "LegendarySupportItem",
            Self::LegendaryTankItem => "LegendaryTankItem",
            Self::LiandrysTorment => "LiandrysTorment",
            Self::LichBane => "LichBane",
            Self::Lifeline => "Lifeline",
            Self::LightningRod => "LightningRod",
            Self::LocketOfTheIronSolari => "LocketOfTheIronSolari",
            Self::LongSword => "LongSword",
            Self::LordDominiksRegards => "LordDominiksRegards",
            Self::LostChapter => "LostChapter",
            Self::LuckyDice => "LuckyDice",
            Self::LudensEcho => "LudensEcho",
            Self::Malignance => "Malignance",
            Self::Manamune => "Manamune",
            Self::MawOfMalmortius => "MawOfMalmortius",
            Self::MejaisSoulstealer => "MejaisSoulstealer",
            Self::MercurialScimitar => "MercurialScimitar",
            Self::MercurysTreads => "MercurysTreads",
            Self::MikaelsBlessing => "MikaelsBlessing",
            Self::MirageBlade => "MirageBlade",
            Self::MoonflairSpellblade => "MoonflairSpellblade",
            Self::MoonstoneRenewer => "MoonstoneRenewer",
            Self::Morellonomicon => "Morellonomicon",
            Self::MortalReminder => "MortalReminder",
            Self::MosstomperSeedling => "MosstomperSeedling",
            Self::Multitool => "Multitool",
            Self::Muramana => "Muramana",
            Self::NashorsTooth => "NashorsTooth",
            Self::NavoriFlickerblade => "NavoriFlickerblade",
            Self::NeedlesslyLargeRod => "NeedlesslyLargeRod",
            Self::NegatronCloak => "NegatronCloak",
            Self::NightHarvester => "NightHarvester",
            Self::Noonquiver => "Noonquiver",
            Self::NullMagicMantle => "NullMagicMantle",
            Self::OblivionOrb => "OblivionOrb",
            Self::OhmwreckerTurretItem => "OhmwreckerTurretItem",
            Self::OracleLens => "OracleLens",
            Self::Overcharged => "Overcharged",
            Self::OverlordsBloodmail => "OverlordsBloodmail",
            Self::Perplexity => "Perplexity",
            Self::Phage => "Phage",
            Self::PhantomDancer => "PhantomDancer",
            Self::PhreakishGusto => "PhreakishGusto",
            Self::Pickaxe => "Pickaxe",
            Self::PlatedSteelcaps => "PlatedSteelcaps",
            Self::PoroSnax => "PoroSnax",
            Self::PrismaticItem => "PrismaticItem",
            Self::PrismaticStatVoucher => "PrismaticStatVoucher",
            Self::ProfaneHydra => "ProfaneHydra",
            Self::ProtoplasmHarness => "ProtoplasmHarness",
            Self::ProwlersClaw => "ProwlersClaw",
            Self::Puppeteer => "Puppeteer",
            Self::PyromancersCloak => "PyromancersCloak",
            Self::QuicksilverSash => "QuicksilverSash",
            Self::RabadonsDeathcap => "RabadonsDeathcap",
            Self::RadiantVirtue => "RadiantVirtue",
            Self::RaiseMorale => "RaiseMorale",
            Self::RanduinsOmen => "RanduinsOmen",
            Self::RapidFirecannon => "RapidFirecannon",
            Self::RavenousHydra => "RavenousHydra",
            Self::RealityFracture => "RealityFracture",
            Self::ReapersToll => "ReapersToll",
            Self::Rectrix => "Rectrix",
            Self::RecurveBow => "RecurveBow",
            Self::Redemption => "Redemption",
            Self::RefillablePotion => "RefillablePotion",
            Self::Regicide => "Regicide",
            Self::ReinforcedArmorTurretItem => "ReinforcedArmorTurretItem",
            Self::RejuvenationBead => "RejuvenationBead",
            Self::Reverberation => "Reverberation",
            Self::Riftmaker => "Riftmaker",
            Self::RiteOfRuin => "RiteOfRuin",
            Self::RodOfAges => "RodOfAges",
            Self::RubyCrystal => "RubyCrystal",
            Self::RunaansHurricane => "RunaansHurricane",
            Self::Runecarver => "Runecarver",
            Self::RunicCompass => "RunicCompass",
            Self::RylaisCrystalScepter => "RylaisCrystalScepter",
            Self::SanguineGift => "SanguineGift",
            Self::SapphireCrystal => "SapphireCrystal",
            Self::ScarecrowEffigy => "ScarecrowEffigy",
            Self::ScorchclawPup => "ScorchclawPup",
            Self::ScoutingAhead => "ScoutingAhead",
            Self::ScoutsSlingshot => "ScoutsSlingshot",
            Self::SeekersArmguard => "SeekersArmguard",
            Self::SeraphsEmbrace => "SeraphsEmbrace",
            Self::SerpentsFang => "SerpentsFang",
            Self::SerratedDirk => "SerratedDirk",
            Self::SeryldasGrudge => "SeryldasGrudge",
            Self::Shadowflame => "Shadowflame",
            Self::ShatteredArmguard => "ShatteredArmguard",
            Self::Sheen => "Sheen",
            Self::ShieldOfMoltenStone => "ShieldOfMoltenStone",
            Self::ShurelyasBattlesong => "ShurelyasBattlesong",
            Self::SlightlyMagicalBoots => "SlightlyMagicalBoots",
            Self::SolsticeSleigh => "SolsticeSleigh",
            Self::SorcerersShoes => "SorcerersShoes",
            Self::SpearOfShojin => "SpearOfShojin",
            Self::SpectralCutlass => "SpectralCutlass",
            Self::SpectresCowl => "SpectresCowl",
            Self::SpellslingersShoes => "SpellslingersShoes",
            Self::SpiritVisage => "SpiritVisage",
            Self::StaffOfFlowingWater => "StaffOfFlowingWater",
            Self::StatBonusAramMayhem => "StatBonusAramMayhem",
            Self::StatBonusArena => "StatBonusArena",
            Self::StatikkShiv => "StatikkShiv",
            Self::StealthWard => "StealthWard",
            Self::SteelSigil => "SteelSigil",
            Self::SteraksGage => "SteraksGage",
            Self::Stormrazor => "Stormrazor",
            Self::Stormsurge => "Stormsurge",
            Self::Stridebreaker => "Stridebreaker",
            Self::SunderedSky => "SunderedSky",
            Self::SunfireAegis => "SunfireAegis",
            Self::SuperMechArmor => "SuperMechArmor",
            Self::SuperMechPowerField => "SuperMechPowerField",
            Self::Swiftmarch => "Swiftmarch",
            Self::SwordOfBlossomingDawn => "SwordOfBlossomingDawn",
            Self::SwordOfTheDivine => "SwordOfTheDivine",
            Self::TalismanOfAscension => "TalismanOfAscension",
            Self::TearOfTheGoddess => "TearOfTheGoddess",
            Self::Terminus => "Terminus",
            Self::TheBrutalizer => "TheBrutalizer",
            Self::TheCollector => "TheCollector",
            Self::TheGoldenSpatula => "TheGoldenSpatula",
            Self::Thornmail => "Thornmail",
            Self::Tiamat => "Tiamat",
            Self::TitanicHydra => "TitanicHydra",
            Self::TotalBiscuitOfEverlastingWill => {
                "TotalBiscuitOfEverlastingWill"
            }
            Self::TowerPowerUp => "TowerPowerUp",
            Self::TrinityForce => "TrinityForce",
            Self::Tunneler => "Tunneler",
            Self::TurboChemtank => "TurboChemtank",
            Self::TurretPlating => "TurretPlating",
            Self::TwilightsEdge => "TwilightsEdge",
            Self::TwinMask => "TwinMask",
            Self::UmbralGlaive => "UmbralGlaive",
            Self::UnendingDespair => "UnendingDespair",
            Self::VampiricScepter => "VampiricScepter",
            Self::VeigarsTalismanOfAscension => "VeigarsTalismanOfAscension",
            Self::VerdantBarrier => "VerdantBarrier",
            Self::VoidImmolation => "VoidImmolation",
            Self::VoidStaff => "VoidStaff",
            Self::VoltaicCyclosword => "VoltaicCyclosword",
            Self::WardensEye => "WardensEye",
            Self::WardensMail => "WardensMail",
            Self::WarmogsArmor => "WarmogsArmor",
            Self::WhisperingCirclet => "WhisperingCirclet",
            Self::WingedMoonplate => "WingedMoonplate",
            Self::WintersApproach => "WintersApproach",
            Self::WitsEnd => "WitsEnd",
            Self::WoogletsWitchcap => "WoogletsWitchcap",
            Self::WordlessPromise => "WordlessPromise",
            Self::WorldAtlas => "WorldAtlas",
            Self::YoumuusGhostblade => "YoumuusGhostblade",
            Self::YourCut => "YourCut",
            Self::YunTalWildarrows => "YunTalWildarrows",
            Self::ZazZaksRealmspike => "ZazZaksRealmspike",
            Self::Zeal => "Zeal",
            Self::ZekesConvergence => "ZekesConvergence",
            Self::Zephyr => "Zephyr",
            Self::ZhonyasHourglass => "ZhonyasHourglass",
        }
    }
    pub const fn from_riot_id(id: u32) -> Option<Self> {
        match id {
            8020 => Some(Self::AbyssalMask),
            2522 => Some(Self::Actualizer),
            3113 => Some(Self::AetherWisp),
            1052 => Some(Self::AmplifyingTome),
            228001 => Some(Self::AnathemasChains),
            1508 => Some(Self::AntiTowerSocks),
            9999 => Some(Self::AnvilVoucher),
            3348 => Some(Self::ArcaneSweeperTrinket),
            3003 => Some(Self::ArchangelsStaff),
            3504 => Some(Self::ArdentCenser),
            3174 => Some(Self::ArmoredAdvance),
            223039 => Some(Self::AtmasReckoning),
            6696 => Some(Self::AxiomArc),
            1038 => Some(Self::BFSword),
            6660 => Some(Self::BamisCinder),
            4642 => Some(Self::BandleglassMirror),
            2524 => Some(Self::Bandlepipes),
            3102 => Some(Self::BansheesVeil),
            1506 => Some(Self::BaseTurretReinforcedArmorTurretItem),
            2520 => Some(Self::Bastionbreaker),
            3006 => Some(Self::BerserkersGreaves),
            3071 => Some(Self::BlackCleaver),
            447122 => Some(Self::BlackHoleGauntlet),
            3599 => Some(Self::BlackSpear),
            2503 => Some(Self::BlackfireTorch),
            3153 => Some(Self::BladeOfTheRuinedKing),
            1026 => Some(Self::BlastingWand),
            4630 => Some(Self::BlightingJewel),
            8010 => Some(Self::BloodlettersCurse),
            3877 => Some(Self::Bloodsong),
            3072 => Some(Self::Bloodthirster),
            1001 => Some(Self::Boots),
            3009 => Some(Self::BootsOfSwiftness),
            3867 => Some(Self::BountyOfWorlds),
            3076 => Some(Self::BrambleVest),
            9999 => Some(Self::BraveryVoucher),
            2141 => Some(Self::CappaJuice),
            3803 => Some(Self::CatalystOfAeons),
            3133 => Some(Self::CaulfieldsWarhammer),
            3869 => Some(Self::CelestialOpposition),
            1031 => Some(Self::ChainVest),
            3173 => Some(Self::ChainlacedCrushers),
            6609 => Some(Self::ChempunkChainsword),
            1018 => Some(Self::CloakOfAgility),
            443059 => Some(Self::CloakOfStarryNight),
            1029 => Some(Self::ClothArmor),
            2055 => Some(Self::ControlWard),
            4629 => Some(Self::CosmicDrive),
            3171 => Some(Self::CrimsonLucidity),
            444644 => Some(Self::CrownOfTheShatteredQueen),
            447109 => Some(Self::Cruelty),
            3137 => Some(Self::Cryptbloom),
            3801 => Some(Self::CrystallineBracer),
            1524 => Some(Self::CrystallineOvergrowth),
            1083 => Some(Self::Cull),
            1042 => Some(Self::Dagger),
            1082 => Some(Self::DarkSeal),
            443054 => Some(Self::DarksteelTalons),
            6621 => Some(Self::Dawncore),
            3742 => Some(Self::DeadMansPlate),
            6333 => Some(Self::DeathsDance),
            3902 => Some(Self::DeathsDaughter),
            447107 => Some(Self::Decapitator),
            443056 => Some(Self::DemonKingsCrown),
            444637 => Some(Self::DemonicEmbrace),
            447113 => Some(Self::DetonationOrb),
            2530 => Some(Self::DiademOfSongs),
            447120 => Some(Self::DiamondTippedSpear),
            446632 => Some(Self::DivineSunderer),
            1055 => Some(Self::DoransBlade),
            1086 => Some(Self::DoransBow),
            1120 => Some(Self::DoransHelm),
            1056 => Some(Self::DoransRing),
            1054 => Some(Self::DoransShield),
            447106 => Some(Self::Dragonheart),
            3870 => Some(Self::DreamMaker),
            2510 => Some(Self::DuskAndDawn),
            446691 => Some(Self::DuskbladeOfDraktharr),
            6620 => Some(Self::EchoesOfHelia),
            6692 => Some(Self::Eclipse),
            3814 => Some(Self::EdgeOfNight),
            443063 => Some(Self::EleisasMiracle),
            2151 => Some(Self::ElixirOfAvarice),
            2152 => Some(Self::ElixirOfForce),
            2138 => Some(Self::ElixirOfIron),
            2150 => Some(Self::ElixirOfSkill),
            2139 => Some(Self::ElixirOfSorcery),
            2140 => Some(Self::ElixirOfWrath),
            447105 => Some(Self::EmpyreanPromise),
            2517 => Some(Self::EndlessHunger),
            2146 => Some(Self::EnhancedLuckyDice),
            3508 => Some(Self::EssenceReaver),
            446656 => Some(Self::Everfrost),
            3123 => Some(Self::ExecutionersCalling),
            3073 => Some(Self::ExperimentalHexplate),
            3513 => Some(Self::EyeOfTheHerald),
            1004 => Some(Self::FaerieCharm),
            3363 => Some(Self::FarsightAlteration),
            2508 => Some(Self::FatedAshes),
            2512 => Some(Self::FiendhunterBolts),
            3108 => Some(Self::FiendishCodex),
            3121 => Some(Self::Fimbulwinter),
            3901 => Some(Self::FireAtWill),
            447112 => Some(Self::Flesheater),
            3114 => Some(Self::ForbiddenIdol),
            443061 => Some(Self::ForceOfEntropy),
            4401 => Some(Self::ForceOfNature),
            999999 => Some(Self::FortificationAram),
            3110 => Some(Self::FrozenHeart),
            443055 => Some(Self::Fulmination),
            446671 => Some(Self::Galeforce),
            447101 => Some(Self::GamblersBlade),
            443193 => Some(Self::GargoyleStoneplate),
            223005 => Some(Self::Ghostcrawlers),
            1011 => Some(Self::GiantsBelt),
            3024 => Some(Self::GlacialBuckler),
            2022 => Some(Self::GlowingMote),
            3008 => Some(Self::GluttonousGreaves),
            9999 => Some(Self::GoldStatAnvilVoucher),
            226630 => Some(Self::Goredrinker),
            3026 => Some(Self::GuardianAngel),
            2049 => Some(Self::GuardiansAmulet),
            3177 => Some(Self::GuardiansBlade),
            223185 => Some(Self::GuardiansDirk),
            3184 => Some(Self::GuardiansHammer),
            2051 => Some(Self::GuardiansHorn),
            3112 => Some(Self::GuardiansOrb),
            2050 => Some(Self::GuardiansShroud),
            3124 => Some(Self::GuinsoosRageblade),
            3172 => Some(Self::GunmetalGreaves),
            1509 => Some(Self::Gusto),
            1102 => Some(Self::GustwalkerHatchling),
            443069 => Some(Self::Hamstringer),
            3147 => Some(Self::HauntingGuise),
            2003 => Some(Self::HealthPotion),
            3051 => Some(Self::HearthboundAxe),
            3084 => Some(Self::Heartsteel),
            4017 => Some(Self::HellfireHatchet),
            447103 => Some(Self::HemomancersHelm),
            443081 => Some(Self::HexboltCompanion),
            3155 => Some(Self::Hexdrinker),
            2523 => Some(Self::HexopticsC44),
            3145 => Some(Self::HextechAlternator),
            3146 => Some(Self::HextechGunblade),
            3152 => Some(Self::HextechRocketbelt),
            6664 => Some(Self::HollowRadiance),
            4628 => Some(Self::HorizonFocus),
            6697 => Some(Self::Hubris),
            3181 => Some(Self::Hullbreaker),
            6662 => Some(Self::IcebornGauntlet),
            3168 => Some(Self::ImmortalPath),
            6673 => Some(Self::ImmortalShieldbow),
            4005 => Some(Self::ImperialMandate),
            3031 => Some(Self::InfinityEdge),
            447104 => Some(Self::InnervatingLocket),
            3158 => Some(Self::IonianBootsOfLucidity),
            6665 => Some(Self::JakShoTheProtean),
            1111 => Some(Self::JarvanIs),
            2144 => Some(Self::JuiceOfHaste),
            2142 => Some(Self::JuiceOfPower),
            2143 => Some(Self::JuiceOfVitality),
            2504 => Some(Self::KaenicRookern),
            3067 => Some(Self::Kindlegem),
            447116 => Some(Self::KinkouJitte),
            3109 => Some(Self::KnightsVow),
            6672 => Some(Self::KrakenSlayer),
            3035 => Some(Self::LastWhisper),
            220003 => Some(Self::LegendaryAssassinItem),
            220001 => Some(Self::LegendaryFighterItem),
            220004 => Some(Self::LegendaryMageItem),
            220002 => Some(Self::LegendaryMarksmanItem),
            220006 => Some(Self::LegendarySupportItem),
            220005 => Some(Self::LegendaryTankItem),
            6653 => Some(Self::LiandrysTorment),
            3100 => Some(Self::LichBane),
            4003 => Some(Self::Lifeline),
            447119 => Some(Self::LightningRod),
            3190 => Some(Self::LocketOfTheIronSolari),
            1036 => Some(Self::LongSword),
            3036 => Some(Self::LordDominiksRegards),
            3802 => Some(Self::LostChapter),
            2145 => Some(Self::LuckyDice),
            6655 => Some(Self::LudensEcho),
            3118 => Some(Self::Malignance),
            3004 => Some(Self::Manamune),
            3156 => Some(Self::MawOfMalmortius),
            3041 => Some(Self::MejaisSoulstealer),
            3139 => Some(Self::MercurialScimitar),
            3111 => Some(Self::MercurysTreads),
            3222 => Some(Self::MikaelsBlessing),
            447100 => Some(Self::MirageBlade),
            447110 => Some(Self::MoonflairSpellblade),
            6617 => Some(Self::MoonstoneRenewer),
            3165 => Some(Self::Morellonomicon),
            3033 => Some(Self::MortalReminder),
            1103 => Some(Self::MosstomperSeedling),
            228009 => Some(Self::Multitool),
            3042 => Some(Self::Muramana),
            3115 => Some(Self::NashorsTooth),
            6675 => Some(Self::NavoriFlickerblade),
            1058 => Some(Self::NeedlesslyLargeRod),
            1057 => Some(Self::NegatronCloak),
            444636 => Some(Self::NightHarvester),
            6670 => Some(Self::Noonquiver),
            1033 => Some(Self::NullMagicMantle),
            3916 => Some(Self::OblivionOrb),
            1500 => Some(Self::OhmwreckerTurretItem),
            3364 => Some(Self::OracleLens),
            1507 => Some(Self::Overcharged),
            2501 => Some(Self::OverlordsBloodmail),
            4015 => Some(Self::Perplexity),
            3044 => Some(Self::Phage),
            3046 => Some(Self::PhantomDancer),
            1510 => Some(Self::PhreakishGusto),
            1037 => Some(Self::Pickaxe),
            3047 => Some(Self::PlatedSteelcaps),
            2052 => Some(Self::PoroSnax),
            220007 => Some(Self::PrismaticItem),
            9999 => Some(Self::PrismaticStatVoucher),
            6698 => Some(Self::ProfaneHydra),
            2525 => Some(Self::ProtoplasmHarness),
            446693 => Some(Self::ProwlersClaw),
            447123 => Some(Self::Puppeteer),
            447118 => Some(Self::PyromancersCloak),
            3140 => Some(Self::QuicksilverSash),
            3089 => Some(Self::RabadonsDeathcap),
            446667 => Some(Self::RadiantVirtue),
            3903 => Some(Self::RaiseMorale),
            3143 => Some(Self::RanduinsOmen),
            3094 => Some(Self::RapidFirecannon),
            3074 => Some(Self::RavenousHydra),
            447102 => Some(Self::RealityFracture),
            443090 => Some(Self::ReapersToll),
            6690 => Some(Self::Rectrix),
            1043 => Some(Self::RecurveBow),
            3107 => Some(Self::Redemption),
            2031 => Some(Self::RefillablePotion),
            447115 => Some(Self::Regicide),
            1502 => Some(Self::ReinforcedArmorTurretItem),
            1006 => Some(Self::RejuvenationBead),
            447114 => Some(Self::Reverberation),
            4633 => Some(Self::Riftmaker),
            3430 => Some(Self::RiteOfRuin),
            6657 => Some(Self::RodOfAges),
            1028 => Some(Self::RubyCrystal),
            3085 => Some(Self::RunaansHurricane),
            447108 => Some(Self::Runecarver),
            3866 => Some(Self::RunicCompass),
            3116 => Some(Self::RylaisCrystalScepter),
            443062 => Some(Self::SanguineGift),
            1027 => Some(Self::SapphireCrystal),
            3330 => Some(Self::ScarecrowEffigy),
            1101 => Some(Self::ScorchclawPup),
            9999 => Some(Self::ScoutingAhead),
            3144 => Some(Self::ScoutsSlingshot),
            2420 => Some(Self::SeekersArmguard),
            3040 => Some(Self::SeraphsEmbrace),
            6695 => Some(Self::SerpentsFang),
            3134 => Some(Self::SerratedDirk),
            6694 => Some(Self::SeryldasGrudge),
            4645 => Some(Self::Shadowflame),
            2421 => Some(Self::ShatteredArmguard),
            3057 => Some(Self::Sheen),
            443058 => Some(Self::ShieldOfMoltenStone),
            2065 => Some(Self::ShurelyasBattlesong),
            2422 => Some(Self::SlightlyMagicalBoots),
            3876 => Some(Self::SolsticeSleigh),
            3020 => Some(Self::SorcerersShoes),
            3161 => Some(Self::SpearOfShojin),
            224004 => Some(Self::SpectralCutlass),
            3211 => Some(Self::SpectresCowl),
            3175 => Some(Self::SpellslingersShoes),
            3065 => Some(Self::SpiritVisage),
            6616 => Some(Self::StaffOfFlowingWater),
            220000 => Some(Self::StatBonusAramMayhem),
            220000 => Some(Self::StatBonusArena),
            3087 => Some(Self::StatikkShiv),
            3340 => Some(Self::StealthWard),
            2019 => Some(Self::SteelSigil),
            3053 => Some(Self::SteraksGage),
            3097 => Some(Self::Stormrazor),
            4646 => Some(Self::Stormsurge),
            6631 => Some(Self::Stridebreaker),
            6610 => Some(Self::SunderedSky),
            3068 => Some(Self::SunfireAegis),
            1511 => Some(Self::SuperMechArmor),
            1512 => Some(Self::SuperMechPowerField),
            3170 => Some(Self::Swiftmarch),
            4011 => Some(Self::SwordOfBlossomingDawn),
            443060 => Some(Self::SwordOfTheDivine),
            443064 => Some(Self::TalismanOfAscension),
            3070 => Some(Self::TearOfTheGoddess),
            3302 => Some(Self::Terminus),
            2020 => Some(Self::TheBrutalizer),
            6676 => Some(Self::TheCollector),
            224403 => Some(Self::TheGoldenSpatula),
            3075 => Some(Self::Thornmail),
            3077 => Some(Self::Tiamat),
            3748 => Some(Self::TitanicHydra),
            2010 => Some(Self::TotalBiscuitOfEverlastingWill),
            999999 => Some(Self::TowerPowerUp),
            3078 => Some(Self::TrinityForce),
            2021 => Some(Self::Tunneler),
            443079 => Some(Self::TurboChemtank),
            1515 => Some(Self::TurretPlating),
            447121 => Some(Self::TwilightsEdge),
            443080 => Some(Self::TwinMask),
            3179 => Some(Self::UmbralGlaive),
            2502 => Some(Self::UnendingDespair),
            1053 => Some(Self::VampiricScepter),
            999999 => Some(Self::VeigarsTalismanOfAscension),
            4632 => Some(Self::VerdantBarrier),
            223069 => Some(Self::VoidImmolation),
            3135 => Some(Self::VoidStaff),
            6699 => Some(Self::VoltaicCyclosword),
            1503 => Some(Self::WardensEye),
            3082 => Some(Self::WardensMail),
            3083 => Some(Self::WarmogsArmor),
            2526 => Some(Self::WhisperingCirclet),
            3066 => Some(Self::WingedMoonplate),
            3119 => Some(Self::WintersApproach),
            3091 => Some(Self::WitsEnd),
            228002 => Some(Self::WoogletsWitchcap),
            4016 => Some(Self::WordlessPromise),
            3865 => Some(Self::WorldAtlas),
            3142 => Some(Self::YoumuusGhostblade),
            3400 => Some(Self::YourCut),
            3032 => Some(Self::YunTalWildarrows),
            3871 => Some(Self::ZazZaksRealmspike),
            3086 => Some(Self::Zeal),
            3050 => Some(Self::ZekesConvergence),
            3172 => Some(Self::Zephyr),
            3157 => Some(Self::ZhonyasHourglass),
            _ => None,
        }
    }
}
pub static ITEM_NAME_TO_ID: phf::Map<&str, ItemId> = phf::phf_map!("ABYSSAL MASK" | "ABYSSALMASK" | "ABYSSAL_MASK" | "Abyssal Mask" | "AbyssalMask" | "Abyssalmask" | "abyssal mask" | "abyssal_mask" | "abyssalmask" => ItemId::AbyssalMask,"ACTUALIZER" | "Actualizer" | "actualizer" => ItemId::Actualizer,"AETHER WISP" | "AETHERWISP" | "AETHER_WISP" | "Aether Wisp" | "AetherWisp" | "Aetherwisp" | "aether wisp" | "aether_wisp" | "aetherwisp" => ItemId::AetherWisp,"AMPLIFYING TOME" | "AMPLIFYINGTOME" | "AMPLIFYING_TOME" | "Amplifying Tome" | "AmplifyingTome" | "Amplifyingtome" | "amplifying tome" | "amplifying_tome" | "amplifyingtome" => ItemId::AmplifyingTome,"ANATHEMA'S CHAINS" | "ANATHEMASCHAINS" | "ANATHEMAS_CHAINS" | "Anathema's Chains" | "AnathemasChains" | "Anathemaschains" | "anathema's chains" | "anathemas_chains" | "anathemaschains" => ItemId::AnathemasChains,"ANTI-TOWER SOCKS" | "ANTITOWERSOCKS" | "ANTI_TOWER_SOCKS" | "Anti-Tower Socks" | "AntiTowerSocks" | "Antitowersocks" | "anti-tower socks" | "anti_tower_socks" | "antitowersocks" => ItemId::AntiTowerSocks,"ANVIL VOUCHER" | "ANVILVOUCHER" | "ANVIL_VOUCHER" | "Anvil Voucher" | "AnvilVoucher" | "Anvilvoucher" | "anvil voucher" | "anvil_voucher" | "anvilvoucher" => ItemId::AnvilVoucher,"ARCANE SWEEPER (TRINKET)" | "ARCANESWEEPERTRINKET" | "ARCANE_SWEEPER_TRINKET" | "Arcane Sweeper (Trinket)" | "ArcaneSweeperTrinket" | "Arcanesweepertrinket" | "arcane sweeper (trinket)" | "arcane_sweeper_trinket" | "arcanesweepertrinket" => ItemId::ArcaneSweeperTrinket,"ARCHANGEL'S STAFF" | "ARCHANGELSSTAFF" | "ARCHANGELS_STAFF" | "Archangel's Staff" | "ArchangelsStaff" | "Archangelsstaff" | "archangel's staff" | "archangels_staff" | "archangelsstaff" => ItemId::ArchangelsStaff,"ARDENT CENSER" | "ARDENTCENSER" | "ARDENT_CENSER" | "Ardent Censer" | "ArdentCenser" | "Ardentcenser" | "ardent censer" | "ardent_censer" | "ardentcenser" => ItemId::ArdentCenser,"ARMORED ADVANCE" | "ARMOREDADVANCE" | "ARMORED_ADVANCE" | "Armored Advance" | "ArmoredAdvance" | "Armoredadvance" | "armored advance" | "armored_advance" | "armoredadvance" => ItemId::ArmoredAdvance,"ATMA'S RECKONING" | "ATMASRECKONING" | "ATMAS_RECKONING" | "Atma's Reckoning" | "AtmasReckoning" | "Atmasreckoning" | "atma's reckoning" | "atmas_reckoning" | "atmasreckoning" => ItemId::AtmasReckoning,"AXIOM ARC" | "AXIOMARC" | "AXIOM_ARC" | "Axiom Arc" | "AxiomArc" | "Axiomarc" | "axiom arc" | "axiom_arc" | "axiomarc" => ItemId::AxiomArc,"B. F. SWORD" | "B. F. Sword" | "BFSWORD" | "BFSword" | "BF_SWORD" | "B_F_SWORD" | "Bfsword" | "b. f. sword" | "b_f_sword" | "bf_sword" | "bfsword" => ItemId::BFSword,"BAMI'S CINDER" | "BAMISCINDER" | "BAMIS_CINDER" | "Bami's Cinder" | "BamisCinder" | "Bamiscinder" | "bami's cinder" | "bamis_cinder" | "bamiscinder" => ItemId::BamisCinder,"BANDLEGLASS MIRROR" | "BANDLEGLASSMIRROR" | "BANDLEGLASS_MIRROR" | "Bandleglass Mirror" | "BandleglassMirror" | "Bandleglassmirror" | "bandleglass mirror" | "bandleglass_mirror" | "bandleglassmirror" => ItemId::BandleglassMirror,"BANDLEPIPES" | "Bandlepipes" | "bandlepipes" => ItemId::Bandlepipes,"BANSHEE'S VEIL" | "BANSHEESVEIL" | "BANSHEES_VEIL" | "Banshee's Veil" | "BansheesVeil" | "Bansheesveil" | "banshee's veil" | "banshees_veil" | "bansheesveil" => ItemId::BansheesVeil,"BASE TURRET REINFORCED ARMOR (TURRET ITEM)" | "BASETURRETREINFORCEDARMORTURRETITEM" | "BASE_TURRET_REINFORCED_ARMOR_TURRET_ITEM" | "Base Turret Reinforced Armor (Turret Item)" | "BaseTurretReinforcedArmorTurretItem" | "Baseturretreinforcedarmorturretitem" | "base turret reinforced armor (turret item)" | "base_turret_reinforced_armor_turret_item" | "baseturretreinforcedarmorturretitem" => ItemId::BaseTurretReinforcedArmorTurretItem,"BASTIONBREAKER" | "Bastionbreaker" | "bastionbreaker" => ItemId::Bastionbreaker,"BERSERKER'S GREAVES" | "BERSERKERSGREAVES" | "BERSERKERS_GREAVES" | "Berserker's Greaves" | "BerserkersGreaves" | "Berserkersgreaves" | "berserker's greaves" | "berserkers_greaves" | "berserkersgreaves" => ItemId::BerserkersGreaves,"BLACK CLEAVER" | "BLACKCLEAVER" | "BLACK_CLEAVER" | "Black Cleaver" | "BlackCleaver" | "Blackcleaver" | "black cleaver" | "black_cleaver" | "blackcleaver" => ItemId::BlackCleaver,"BLACK HOLE GAUNTLET" | "BLACKHOLEGAUNTLET" | "BLACK_HOLE_GAUNTLET" | "Black Hole Gauntlet" | "BlackHoleGauntlet" | "Blackholegauntlet" | "black hole gauntlet" | "black_hole_gauntlet" | "blackholegauntlet" => ItemId::BlackHoleGauntlet,"BLACK SPEAR" | "BLACKSPEAR" | "BLACK_SPEAR" | "Black Spear" | "BlackSpear" | "Blackspear" | "black spear" | "black_spear" | "blackspear" => ItemId::BlackSpear,"BLACKFIRE TORCH" | "BLACKFIRETORCH" | "BLACKFIRE_TORCH" | "Blackfire Torch" | "BlackfireTorch" | "Blackfiretorch" | "blackfire torch" | "blackfire_torch" | "blackfiretorch" => ItemId::BlackfireTorch,"BLADE OF THE RUINED KING" | "BLADEOFTHERUINEDKING" | "BLADE_OF_THE_RUINED_KING" | "Blade of the Ruined King" | "BladeOfTheRuinedKing" | "Bladeoftheruinedking" | "blade of the ruined king" | "blade_of_the_ruined_king" | "bladeoftheruinedking" => ItemId::BladeOfTheRuinedKing,"BLASTING WAND" | "BLASTINGWAND" | "BLASTING_WAND" | "Blasting Wand" | "BlastingWand" | "Blastingwand" | "blasting wand" | "blasting_wand" | "blastingwand" => ItemId::BlastingWand,"BLIGHTING JEWEL" | "BLIGHTINGJEWEL" | "BLIGHTING_JEWEL" | "Blighting Jewel" | "BlightingJewel" | "Blightingjewel" | "blighting jewel" | "blighting_jewel" | "blightingjewel" => ItemId::BlightingJewel,"BLOODLETTER'S CURSE" | "BLOODLETTERSCURSE" | "BLOODLETTERS_CURSE" | "Bloodletter's Curse" | "BloodlettersCurse" | "Bloodletterscurse" | "bloodletter's curse" | "bloodletters_curse" | "bloodletterscurse" => ItemId::BloodlettersCurse,"BLOODSONG" | "Bloodsong" | "bloodsong" => ItemId::Bloodsong,"BLOODTHIRSTER" | "Bloodthirster" | "bloodthirster" => ItemId::Bloodthirster,"BOOTS" | "Boots" | "boots" => ItemId::Boots,"BOOTS OF SWIFTNESS" | "BOOTSOFSWIFTNESS" | "BOOTS_OF_SWIFTNESS" | "Boots of Swiftness" | "BootsOfSwiftness" | "Bootsofswiftness" | "boots of swiftness" | "boots_of_swiftness" | "bootsofswiftness" => ItemId::BootsOfSwiftness,"BOUNTY OF WORLDS" | "BOUNTYOFWORLDS" | "BOUNTY_OF_WORLDS" | "Bounty of Worlds" | "BountyOfWorlds" | "Bountyofworlds" | "bounty of worlds" | "bounty_of_worlds" | "bountyofworlds" => ItemId::BountyOfWorlds,"BRAMBLE VEST" | "BRAMBLEVEST" | "BRAMBLE_VEST" | "Bramble Vest" | "BrambleVest" | "Bramblevest" | "bramble vest" | "bramble_vest" | "bramblevest" => ItemId::BrambleVest,"BRAVERY VOUCHER" | "BRAVERYVOUCHER" | "BRAVERY_VOUCHER" | "Bravery Voucher" | "BraveryVoucher" | "Braveryvoucher" | "bravery voucher" | "bravery_voucher" | "braveryvoucher" => ItemId::BraveryVoucher,"CAPPA JUICE" | "CAPPAJUICE" | "CAPPA_JUICE" | "Cappa Juice" | "CappaJuice" | "Cappajuice" | "cappa juice" | "cappa_juice" | "cappajuice" => ItemId::CappaJuice,"CATALYST OF AEONS" | "CATALYSTOFAEONS" | "CATALYST_OF_AEONS" | "Catalyst of Aeons" | "CatalystOfAeons" | "Catalystofaeons" | "catalyst of aeons" | "catalyst_of_aeons" | "catalystofaeons" => ItemId::CatalystOfAeons,"CAULFIELD'S WARHAMMER" | "CAULFIELDSWARHAMMER" | "CAULFIELDS_WARHAMMER" | "Caulfield's Warhammer" | "CaulfieldsWarhammer" | "Caulfieldswarhammer" | "caulfield's warhammer" | "caulfields_warhammer" | "caulfieldswarhammer" => ItemId::CaulfieldsWarhammer,"CELESTIAL OPPOSITION" | "CELESTIALOPPOSITION" | "CELESTIAL_OPPOSITION" | "Celestial Opposition" | "CelestialOpposition" | "Celestialopposition" | "celestial opposition" | "celestial_opposition" | "celestialopposition" => ItemId::CelestialOpposition,"CHAIN VEST" | "CHAINVEST" | "CHAIN_VEST" | "Chain Vest" | "ChainVest" | "Chainvest" | "chain vest" | "chain_vest" | "chainvest" => ItemId::ChainVest,"CHAINLACED CRUSHERS" | "CHAINLACEDCRUSHERS" | "CHAINLACED_CRUSHERS" | "Chainlaced Crushers" | "ChainlacedCrushers" | "Chainlacedcrushers" | "chainlaced crushers" | "chainlaced_crushers" | "chainlacedcrushers" => ItemId::ChainlacedCrushers,"CHEMPUNK CHAINSWORD" | "CHEMPUNKCHAINSWORD" | "CHEMPUNK_CHAINSWORD" | "Chempunk Chainsword" | "ChempunkChainsword" | "Chempunkchainsword" | "chempunk chainsword" | "chempunk_chainsword" | "chempunkchainsword" => ItemId::ChempunkChainsword,"CLOAK OF AGILITY" | "CLOAKOFAGILITY" | "CLOAK_OF_AGILITY" | "Cloak of Agility" | "CloakOfAgility" | "Cloakofagility" | "cloak of agility" | "cloak_of_agility" | "cloakofagility" => ItemId::CloakOfAgility,"CLOAK OF STARRY NIGHT" | "CLOAKOFSTARRYNIGHT" | "CLOAK_OF_STARRY_NIGHT" | "Cloak of Starry Night" | "CloakOfStarryNight" | "Cloakofstarrynight" | "cloak of starry night" | "cloak_of_starry_night" | "cloakofstarrynight" => ItemId::CloakOfStarryNight,"CLOTH ARMOR" | "CLOTHARMOR" | "CLOTH_ARMOR" | "Cloth Armor" | "ClothArmor" | "Clotharmor" | "cloth armor" | "cloth_armor" | "clotharmor" => ItemId::ClothArmor,"CONTROL WARD" | "CONTROLWARD" | "CONTROL_WARD" | "Control Ward" | "ControlWard" | "Controlward" | "control ward" | "control_ward" | "controlward" => ItemId::ControlWard,"COSMIC DRIVE" | "COSMICDRIVE" | "COSMIC_DRIVE" | "Cosmic Drive" | "CosmicDrive" | "Cosmicdrive" | "cosmic drive" | "cosmic_drive" | "cosmicdrive" => ItemId::CosmicDrive,"CRIMSON LUCIDITY" | "CRIMSONLUCIDITY" | "CRIMSON_LUCIDITY" | "Crimson Lucidity" | "CrimsonLucidity" | "Crimsonlucidity" | "crimson lucidity" | "crimson_lucidity" | "crimsonlucidity" => ItemId::CrimsonLucidity,"CROWN OF THE SHATTERED QUEEN" | "CROWNOFTHESHATTEREDQUEEN" | "CROWN_OF_THE_SHATTERED_QUEEN" | "Crown of the Shattered Queen" | "CrownOfTheShatteredQueen" | "Crownoftheshatteredqueen" | "crown of the shattered queen" | "crown_of_the_shattered_queen" | "crownoftheshatteredqueen" => ItemId::CrownOfTheShatteredQueen,"CRUELTY" | "Cruelty" | "cruelty" => ItemId::Cruelty,"CRYPTBLOOM" | "Cryptbloom" | "cryptbloom" => ItemId::Cryptbloom,"CRYSTALLINE BRACER" | "CRYSTALLINEBRACER" | "CRYSTALLINE_BRACER" | "Crystalline Bracer" | "CrystallineBracer" | "Crystallinebracer" | "crystalline bracer" | "crystalline_bracer" | "crystallinebracer" => ItemId::CrystallineBracer,"CRYSTALLINE OVERGROWTH" | "CRYSTALLINEOVERGROWTH" | "CRYSTALLINE_OVERGROWTH" | "Crystalline Overgrowth" | "CrystallineOvergrowth" | "Crystallineovergrowth" | "crystalline overgrowth" | "crystalline_overgrowth" | "crystallineovergrowth" => ItemId::CrystallineOvergrowth,"CULL" | "Cull" | "cull" => ItemId::Cull,"DAGGER" | "Dagger" | "dagger" => ItemId::Dagger,"DARK SEAL" | "DARKSEAL" | "DARK_SEAL" | "Dark Seal" | "DarkSeal" | "Darkseal" | "dark seal" | "dark_seal" | "darkseal" => ItemId::DarkSeal,"DARKSTEEL TALONS" | "DARKSTEELTALONS" | "DARKSTEEL_TALONS" | "Darksteel Talons" | "DarksteelTalons" | "Darksteeltalons" | "darksteel talons" | "darksteel_talons" | "darksteeltalons" => ItemId::DarksteelTalons,"DAWNCORE" | "Dawncore" | "dawncore" => ItemId::Dawncore,"DEAD MAN'S PLATE" | "DEADMANSPLATE" | "DEAD_MANS_PLATE" | "Dead Man's Plate" | "DeadMansPlate" | "Deadmansplate" | "dead man's plate" | "dead_mans_plate" | "deadmansplate" => ItemId::DeadMansPlate,"DEATH'S DANCE" | "DEATHSDANCE" | "DEATHS_DANCE" | "Death's Dance" | "DeathsDance" | "Deathsdance" | "death's dance" | "deaths_dance" | "deathsdance" => ItemId::DeathsDance,"DEATH'S DAUGHTER" | "DEATHSDAUGHTER" | "DEATHS_DAUGHTER" | "Death's Daughter" | "DeathsDaughter" | "Deathsdaughter" | "death's daughter" | "deaths_daughter" | "deathsdaughter" => ItemId::DeathsDaughter,"DECAPITATOR" | "Decapitator" | "decapitator" => ItemId::Decapitator,"DEMON KING'S CROWN" | "DEMONKINGSCROWN" | "DEMON_KINGS_CROWN" | "Demon King's Crown" | "DemonKingsCrown" | "Demonkingscrown" | "demon king's crown" | "demon_kings_crown" | "demonkingscrown" => ItemId::DemonKingsCrown,"DEMONIC EMBRACE" | "DEMONICEMBRACE" | "DEMONIC_EMBRACE" | "Demonic Embrace" | "DemonicEmbrace" | "Demonicembrace" | "demonic embrace" | "demonic_embrace" | "demonicembrace" => ItemId::DemonicEmbrace,"DETONATION ORB" | "DETONATIONORB" | "DETONATION_ORB" | "Detonation Orb" | "DetonationOrb" | "Detonationorb" | "detonation orb" | "detonation_orb" | "detonationorb" => ItemId::DetonationOrb,"DIADEM OF SONGS" | "DIADEMOFSONGS" | "DIADEM_OF_SONGS" | "Diadem of Songs" | "DiademOfSongs" | "Diademofsongs" | "diadem of songs" | "diadem_of_songs" | "diademofsongs" => ItemId::DiademOfSongs,"DIAMOND-TIPPED SPEAR" | "DIAMONDTIPPEDSPEAR" | "DIAMOND_TIPPED_SPEAR" | "Diamond-Tipped Spear" | "DiamondTippedSpear" | "Diamondtippedspear" | "diamond-tipped spear" | "diamond_tipped_spear" | "diamondtippedspear" => ItemId::DiamondTippedSpear,"DIVINE SUNDERER" | "DIVINESUNDERER" | "DIVINE_SUNDERER" | "Divine Sunderer" | "DivineSunderer" | "Divinesunderer" | "divine sunderer" | "divine_sunderer" | "divinesunderer" => ItemId::DivineSunderer,"DORAN'S BLADE" | "DORANSBLADE" | "DORANS_BLADE" | "Doran's Blade" | "DoransBlade" | "Doransblade" | "doran's blade" | "dorans_blade" | "doransblade" => ItemId::DoransBlade,"DORAN'S BOW" | "DORANSBOW" | "DORANS_BOW" | "Doran's Bow" | "DoransBow" | "Doransbow" | "doran's bow" | "dorans_bow" | "doransbow" => ItemId::DoransBow,"DORAN'S HELM" | "DORANSHELM" | "DORANS_HELM" | "Doran's Helm" | "DoransHelm" | "Doranshelm" | "doran's helm" | "dorans_helm" | "doranshelm" => ItemId::DoransHelm,"DORAN'S RING" | "DORANSRING" | "DORANS_RING" | "Doran's Ring" | "DoransRing" | "Doransring" | "doran's ring" | "dorans_ring" | "doransring" => ItemId::DoransRing,"DORAN'S SHIELD" | "DORANSSHIELD" | "DORANS_SHIELD" | "Doran's Shield" | "DoransShield" | "Doransshield" | "doran's shield" | "dorans_shield" | "doransshield" => ItemId::DoransShield,"DRAGONHEART" | "Dragonheart" | "dragonheart" => ItemId::Dragonheart,"DREAM MAKER" | "DREAMMAKER" | "DREAM_MAKER" | "Dream Maker" | "DreamMaker" | "Dreammaker" | "dream maker" | "dream_maker" | "dreammaker" => ItemId::DreamMaker,"DUSK AND DAWN" | "DUSKANDDAWN" | "DUSK_AND_DAWN" | "Dusk and Dawn" | "DuskAndDawn" | "Duskanddawn" | "dusk and dawn" | "dusk_and_dawn" | "duskanddawn" => ItemId::DuskAndDawn,"DUSKBLADE OF DRAKTHARR" | "DUSKBLADEOFDRAKTHARR" | "DUSKBLADE_OF_DRAKTHARR" | "Duskblade of Draktharr" | "DuskbladeOfDraktharr" | "Duskbladeofdraktharr" | "duskblade of draktharr" | "duskblade_of_draktharr" | "duskbladeofdraktharr" => ItemId::DuskbladeOfDraktharr,"ECHOES OF HELIA" | "ECHOESOFHELIA" | "ECHOES_OF_HELIA" | "Echoes of Helia" | "EchoesOfHelia" | "Echoesofhelia" | "echoes of helia" | "echoes_of_helia" | "echoesofhelia" => ItemId::EchoesOfHelia,"ECLIPSE" | "Eclipse" | "eclipse" => ItemId::Eclipse,"EDGE OF NIGHT" | "EDGEOFNIGHT" | "EDGE_OF_NIGHT" | "Edge of Night" | "EdgeOfNight" | "Edgeofnight" | "edge of night" | "edge_of_night" | "edgeofnight" => ItemId::EdgeOfNight,"ELEISA'S MIRACLE" | "ELEISASMIRACLE" | "ELEISAS_MIRACLE" | "Eleisa's Miracle" | "EleisasMiracle" | "Eleisasmiracle" | "eleisa's miracle" | "eleisas_miracle" | "eleisasmiracle" => ItemId::EleisasMiracle,"ELIXIR OF AVARICE" | "ELIXIROFAVARICE" | "ELIXIR_OF_AVARICE" | "Elixir of Avarice" | "ElixirOfAvarice" | "Elixirofavarice" | "elixir of avarice" | "elixir_of_avarice" | "elixirofavarice" => ItemId::ElixirOfAvarice,"ELIXIR OF FORCE" | "ELIXIROFFORCE" | "ELIXIR_OF_FORCE" | "Elixir of Force" | "ElixirOfForce" | "Elixirofforce" | "elixir of force" | "elixir_of_force" | "elixirofforce" => ItemId::ElixirOfForce,"ELIXIR OF IRON" | "ELIXIROFIRON" | "ELIXIR_OF_IRON" | "Elixir of Iron" | "ElixirOfIron" | "Elixirofiron" | "elixir of iron" | "elixir_of_iron" | "elixirofiron" => ItemId::ElixirOfIron,"ELIXIR OF SKILL" | "ELIXIROFSKILL" | "ELIXIR_OF_SKILL" | "Elixir of Skill" | "ElixirOfSkill" | "Elixirofskill" | "elixir of skill" | "elixir_of_skill" | "elixirofskill" => ItemId::ElixirOfSkill,"ELIXIR OF SORCERY" | "ELIXIROFSORCERY" | "ELIXIR_OF_SORCERY" | "Elixir of Sorcery" | "ElixirOfSorcery" | "Elixirofsorcery" | "elixir of sorcery" | "elixir_of_sorcery" | "elixirofsorcery" => ItemId::ElixirOfSorcery,"ELIXIR OF WRATH" | "ELIXIROFWRATH" | "ELIXIR_OF_WRATH" | "Elixir of Wrath" | "ElixirOfWrath" | "Elixirofwrath" | "elixir of wrath" | "elixir_of_wrath" | "elixirofwrath" => ItemId::ElixirOfWrath,"EMPYREAN PROMISE" | "EMPYREANPROMISE" | "EMPYREAN_PROMISE" | "Empyrean Promise" | "EmpyreanPromise" | "Empyreanpromise" | "empyrean promise" | "empyrean_promise" | "empyreanpromise" => ItemId::EmpyreanPromise,"ENDLESS HUNGER" | "ENDLESSHUNGER" | "ENDLESS_HUNGER" | "Endless Hunger" | "EndlessHunger" | "Endlesshunger" | "endless hunger" | "endless_hunger" | "endlesshunger" => ItemId::EndlessHunger,"ENHANCED LUCKY DICE" | "ENHANCEDLUCKYDICE" | "ENHANCED_LUCKY_DICE" | "Enhanced Lucky Dice" | "EnhancedLuckyDice" | "Enhancedluckydice" | "enhanced lucky dice" | "enhanced_lucky_dice" | "enhancedluckydice" => ItemId::EnhancedLuckyDice,"ESSENCE REAVER" | "ESSENCEREAVER" | "ESSENCE_REAVER" | "Essence Reaver" | "EssenceReaver" | "Essencereaver" | "essence reaver" | "essence_reaver" | "essencereaver" => ItemId::EssenceReaver,"EVERFROST" | "Everfrost" | "everfrost" => ItemId::Everfrost,"EXECUTIONER'S CALLING" | "EXECUTIONERSCALLING" | "EXECUTIONERS_CALLING" | "Executioner's Calling" | "ExecutionersCalling" | "Executionerscalling" | "executioner's calling" | "executioners_calling" | "executionerscalling" => ItemId::ExecutionersCalling,"EXPERIMENTAL HEXPLATE" | "EXPERIMENTALHEXPLATE" | "EXPERIMENTAL_HEXPLATE" | "Experimental Hexplate" | "ExperimentalHexplate" | "Experimentalhexplate" | "experimental hexplate" | "experimental_hexplate" | "experimentalhexplate" => ItemId::ExperimentalHexplate,"EYE OF THE HERALD" | "EYEOFTHEHERALD" | "EYE_OF_THE_HERALD" | "Eye of the Herald" | "EyeOfTheHerald" | "Eyeoftheherald" | "eye of the herald" | "eye_of_the_herald" | "eyeoftheherald" => ItemId::EyeOfTheHerald,"FAERIE CHARM" | "FAERIECHARM" | "FAERIE_CHARM" | "Faerie Charm" | "FaerieCharm" | "Faeriecharm" | "faerie charm" | "faerie_charm" | "faeriecharm" => ItemId::FaerieCharm,"FARSIGHT ALTERATION" | "FARSIGHTALTERATION" | "FARSIGHT_ALTERATION" | "Farsight Alteration" | "FarsightAlteration" | "Farsightalteration" | "farsight alteration" | "farsight_alteration" | "farsightalteration" => ItemId::FarsightAlteration,"FATED ASHES" | "FATEDASHES" | "FATED_ASHES" | "Fated Ashes" | "FatedAshes" | "Fatedashes" | "fated ashes" | "fated_ashes" | "fatedashes" => ItemId::FatedAshes,"FIENDHUNTER BOLTS" | "FIENDHUNTERBOLTS" | "FIENDHUNTER_BOLTS" | "Fiendhunter Bolts" | "FiendhunterBolts" | "Fiendhunterbolts" | "fiendhunter bolts" | "fiendhunter_bolts" | "fiendhunterbolts" => ItemId::FiendhunterBolts,"FIENDISH CODEX" | "FIENDISHCODEX" | "FIENDISH_CODEX" | "Fiendish Codex" | "FiendishCodex" | "Fiendishcodex" | "fiendish codex" | "fiendish_codex" | "fiendishcodex" => ItemId::FiendishCodex,"FIMBULWINTER" | "Fimbulwinter" | "fimbulwinter" => ItemId::Fimbulwinter,"FIRE AT WILL" | "FIREATWILL" | "FIRE_AT_WILL" | "Fire at Will" | "FireAtWill" | "Fireatwill" | "fire at will" | "fire_at_will" | "fireatwill" => ItemId::FireAtWill,"FLESHEATER" | "Flesheater" | "flesheater" => ItemId::Flesheater,"FORBIDDEN IDOL" | "FORBIDDENIDOL" | "FORBIDDEN_IDOL" | "Forbidden Idol" | "ForbiddenIdol" | "Forbiddenidol" | "forbidden idol" | "forbidden_idol" | "forbiddenidol" => ItemId::ForbiddenIdol,"FORCE OF ENTROPY" | "FORCEOFENTROPY" | "FORCE_OF_ENTROPY" | "Force of Entropy" | "ForceOfEntropy" | "Forceofentropy" | "force of entropy" | "force_of_entropy" | "forceofentropy" => ItemId::ForceOfEntropy,"FORCE OF NATURE" | "FORCEOFNATURE" | "FORCE_OF_NATURE" | "Force of Nature" | "ForceOfNature" | "Forceofnature" | "force of nature" | "force_of_nature" | "forceofnature" => ItemId::ForceOfNature,"FORTIFICATION (ARAM)" | "FORTIFICATIONARAM" | "FORTIFICATION_ARAM" | "Fortification (ARAM)" | "FortificationAram" | "Fortificationaram" | "fortification (aram)" | "fortification_aram" | "fortificationaram" => ItemId::FortificationAram,"FROZEN HEART" | "FROZENHEART" | "FROZEN_HEART" | "Frozen Heart" | "FrozenHeart" | "Frozenheart" | "frozen heart" | "frozen_heart" | "frozenheart" => ItemId::FrozenHeart,"FULMINATION" | "Fulmination" | "fulmination" => ItemId::Fulmination,"GALEFORCE" | "Galeforce" | "galeforce" => ItemId::Galeforce,"GAMBLER'S BLADE" | "GAMBLERSBLADE" | "GAMBLERS_BLADE" | "Gambler's Blade" | "GamblersBlade" | "Gamblersblade" | "gambler's blade" | "gamblers_blade" | "gamblersblade" => ItemId::GamblersBlade,"GARGOYLE STONEPLATE" | "GARGOYLESTONEPLATE" | "GARGOYLE_STONEPLATE" | "Gargoyle Stoneplate" | "GargoyleStoneplate" | "Gargoylestoneplate" | "gargoyle stoneplate" | "gargoyle_stoneplate" | "gargoylestoneplate" => ItemId::GargoyleStoneplate,"GHOSTCRAWLERS" | "Ghostcrawlers" | "ghostcrawlers" => ItemId::Ghostcrawlers,"GIANT'S BELT" | "GIANTSBELT" | "GIANTS_BELT" | "Giant's Belt" | "GiantsBelt" | "Giantsbelt" | "giant's belt" | "giants_belt" | "giantsbelt" => ItemId::GiantsBelt,"GLACIAL BUCKLER" | "GLACIALBUCKLER" | "GLACIAL_BUCKLER" | "Glacial Buckler" | "GlacialBuckler" | "Glacialbuckler" | "glacial buckler" | "glacial_buckler" | "glacialbuckler" => ItemId::GlacialBuckler,"GLOWING MOTE" | "GLOWINGMOTE" | "GLOWING_MOTE" | "Glowing Mote" | "GlowingMote" | "Glowingmote" | "glowing mote" | "glowing_mote" | "glowingmote" => ItemId::GlowingMote,"GLUTTONOUS GREAVES" | "GLUTTONOUSGREAVES" | "GLUTTONOUS_GREAVES" | "Gluttonous Greaves" | "GluttonousGreaves" | "Gluttonousgreaves" | "gluttonous greaves" | "gluttonous_greaves" | "gluttonousgreaves" => ItemId::GluttonousGreaves,"GOLD STAT ANVIL VOUCHER" | "GOLDSTATANVILVOUCHER" | "GOLD_STAT_ANVIL_VOUCHER" | "Gold Stat Anvil Voucher" | "GoldStatAnvilVoucher" | "Goldstatanvilvoucher" | "gold stat anvil voucher" | "gold_stat_anvil_voucher" | "goldstatanvilvoucher" => ItemId::GoldStatAnvilVoucher,"GOREDRINKER" | "Goredrinker" | "goredrinker" => ItemId::Goredrinker,"GUARDIAN ANGEL" | "GUARDIANANGEL" | "GUARDIAN_ANGEL" | "Guardian Angel" | "GuardianAngel" | "Guardianangel" | "guardian angel" | "guardian_angel" | "guardianangel" => ItemId::GuardianAngel,"GUARDIAN'S AMULET" | "GUARDIANSAMULET" | "GUARDIANS_AMULET" | "Guardian's Amulet" | "GuardiansAmulet" | "Guardiansamulet" | "guardian's amulet" | "guardians_amulet" | "guardiansamulet" => ItemId::GuardiansAmulet,"GUARDIAN'S BLADE" | "GUARDIANSBLADE" | "GUARDIANS_BLADE" | "Guardian's Blade" | "GuardiansBlade" | "Guardiansblade" | "guardian's blade" | "guardians_blade" | "guardiansblade" => ItemId::GuardiansBlade,"GUARDIAN'S DIRK" | "GUARDIANSDIRK" | "GUARDIANS_DIRK" | "Guardian's Dirk" | "GuardiansDirk" | "Guardiansdirk" | "guardian's dirk" | "guardians_dirk" | "guardiansdirk" => ItemId::GuardiansDirk,"GUARDIAN'S HAMMER" | "GUARDIANSHAMMER" | "GUARDIANS_HAMMER" | "Guardian's Hammer" | "GuardiansHammer" | "Guardianshammer" | "guardian's hammer" | "guardians_hammer" | "guardianshammer" => ItemId::GuardiansHammer,"GUARDIAN'S HORN" | "GUARDIANSHORN" | "GUARDIANS_HORN" | "Guardian's Horn" | "GuardiansHorn" | "Guardianshorn" | "guardian's horn" | "guardians_horn" | "guardianshorn" => ItemId::GuardiansHorn,"GUARDIAN'S ORB" | "GUARDIANSORB" | "GUARDIANS_ORB" | "Guardian's Orb" | "GuardiansOrb" | "Guardiansorb" | "guardian's orb" | "guardians_orb" | "guardiansorb" => ItemId::GuardiansOrb,"GUARDIAN'S SHROUD" | "GUARDIANSSHROUD" | "GUARDIANS_SHROUD" | "Guardian's Shroud" | "GuardiansShroud" | "Guardiansshroud" | "guardian's shroud" | "guardians_shroud" | "guardiansshroud" => ItemId::GuardiansShroud,"GUINSOO'S RAGEBLADE" | "GUINSOOSRAGEBLADE" | "GUINSOOS_RAGEBLADE" | "Guinsoo's Rageblade" | "GuinsoosRageblade" | "Guinsoosrageblade" | "guinsoo's rageblade" | "guinsoos_rageblade" | "guinsoosrageblade" => ItemId::GuinsoosRageblade,"GUNMETAL GREAVES" | "GUNMETALGREAVES" | "GUNMETAL_GREAVES" | "Gunmetal Greaves" | "GunmetalGreaves" | "Gunmetalgreaves" | "gunmetal greaves" | "gunmetal_greaves" | "gunmetalgreaves" => ItemId::GunmetalGreaves,"GUSTO" | "Gusto" | "gusto" => ItemId::Gusto,"GUSTWALKER HATCHLING" | "GUSTWALKERHATCHLING" | "GUSTWALKER_HATCHLING" | "Gustwalker Hatchling" | "GustwalkerHatchling" | "Gustwalkerhatchling" | "gustwalker hatchling" | "gustwalker_hatchling" | "gustwalkerhatchling" => ItemId::GustwalkerHatchling,"HAMSTRINGER" | "Hamstringer" | "hamstringer" => ItemId::Hamstringer,"HAUNTING GUISE" | "HAUNTINGGUISE" | "HAUNTING_GUISE" | "Haunting Guise" | "HauntingGuise" | "Hauntingguise" | "haunting guise" | "haunting_guise" | "hauntingguise" => ItemId::HauntingGuise,"HEALTH POTION" | "HEALTHPOTION" | "HEALTH_POTION" | "Health Potion" | "HealthPotion" | "Healthpotion" | "health potion" | "health_potion" | "healthpotion" => ItemId::HealthPotion,"HEARTHBOUND AXE" | "HEARTHBOUNDAXE" | "HEARTHBOUND_AXE" | "Hearthbound Axe" | "HearthboundAxe" | "Hearthboundaxe" | "hearthbound axe" | "hearthbound_axe" | "hearthboundaxe" => ItemId::HearthboundAxe,"HEARTSTEEL" | "Heartsteel" | "heartsteel" => ItemId::Heartsteel,"HELLFIRE HATCHET" | "HELLFIREHATCHET" | "HELLFIRE_HATCHET" | "Hellfire Hatchet" | "HellfireHatchet" | "Hellfirehatchet" | "hellfire hatchet" | "hellfire_hatchet" | "hellfirehatchet" => ItemId::HellfireHatchet,"HEMOMANCER'S HELM" | "HEMOMANCERSHELM" | "HEMOMANCERS_HELM" | "Hemomancer's Helm" | "HemomancersHelm" | "Hemomancershelm" | "hemomancer's helm" | "hemomancers_helm" | "hemomancershelm" => ItemId::HemomancersHelm,"HEXBOLT COMPANION" | "HEXBOLTCOMPANION" | "HEXBOLT_COMPANION" | "Hexbolt Companion" | "HexboltCompanion" | "Hexboltcompanion" | "hexbolt companion" | "hexbolt_companion" | "hexboltcompanion" => ItemId::HexboltCompanion,"HEXDRINKER" | "Hexdrinker" | "hexdrinker" => ItemId::Hexdrinker,"HEXOPTICS C44" | "HEXOPTICSC44" | "HEXOPTICS_C_44" | "Hexoptics C44" | "HexopticsC44" | "Hexopticsc44" | "hexoptics c44" | "hexoptics_c_44" | "hexopticsc44" => ItemId::HexopticsC44,"HEXTECH ALTERNATOR" | "HEXTECHALTERNATOR" | "HEXTECH_ALTERNATOR" | "Hextech Alternator" | "HextechAlternator" | "Hextechalternator" | "hextech alternator" | "hextech_alternator" | "hextechalternator" => ItemId::HextechAlternator,"HEXTECH GUNBLADE" | "HEXTECHGUNBLADE" | "HEXTECH_GUNBLADE" | "Hextech Gunblade" | "HextechGunblade" | "Hextechgunblade" | "hextech gunblade" | "hextech_gunblade" | "hextechgunblade" => ItemId::HextechGunblade,"HEXTECH ROCKETBELT" | "HEXTECHROCKETBELT" | "HEXTECH_ROCKETBELT" | "Hextech Rocketbelt" | "HextechRocketbelt" | "Hextechrocketbelt" | "hextech rocketbelt" | "hextech_rocketbelt" | "hextechrocketbelt" => ItemId::HextechRocketbelt,"HOLLOW RADIANCE" | "HOLLOWRADIANCE" | "HOLLOW_RADIANCE" | "Hollow Radiance" | "HollowRadiance" | "Hollowradiance" | "hollow radiance" | "hollow_radiance" | "hollowradiance" => ItemId::HollowRadiance,"HORIZON FOCUS" | "HORIZONFOCUS" | "HORIZON_FOCUS" | "Horizon Focus" | "HorizonFocus" | "Horizonfocus" | "horizon focus" | "horizon_focus" | "horizonfocus" => ItemId::HorizonFocus,"HUBRIS" | "Hubris" | "hubris" => ItemId::Hubris,"HULLBREAKER" | "Hullbreaker" | "hullbreaker" => ItemId::Hullbreaker,"ICEBORN GAUNTLET" | "ICEBORNGAUNTLET" | "ICEBORN_GAUNTLET" | "Iceborn Gauntlet" | "IcebornGauntlet" | "Iceborngauntlet" | "iceborn gauntlet" | "iceborn_gauntlet" | "iceborngauntlet" => ItemId::IcebornGauntlet,"IMMORTAL PATH" | "IMMORTALPATH" | "IMMORTAL_PATH" | "Immortal Path" | "ImmortalPath" | "Immortalpath" | "immortal path" | "immortal_path" | "immortalpath" => ItemId::ImmortalPath,"IMMORTAL SHIELDBOW" | "IMMORTALSHIELDBOW" | "IMMORTAL_SHIELDBOW" | "Immortal Shieldbow" | "ImmortalShieldbow" | "Immortalshieldbow" | "immortal shieldbow" | "immortal_shieldbow" | "immortalshieldbow" => ItemId::ImmortalShieldbow,"IMPERIAL MANDATE" | "IMPERIALMANDATE" | "IMPERIAL_MANDATE" | "Imperial Mandate" | "ImperialMandate" | "Imperialmandate" | "imperial mandate" | "imperial_mandate" | "imperialmandate" => ItemId::ImperialMandate,"INFINITY EDGE" | "INFINITYEDGE" | "INFINITY_EDGE" | "Infinity Edge" | "InfinityEdge" | "Infinityedge" | "infinity edge" | "infinity_edge" | "infinityedge" => ItemId::InfinityEdge,"INNERVATING LOCKET" | "INNERVATINGLOCKET" | "INNERVATING_LOCKET" | "Innervating Locket" | "InnervatingLocket" | "Innervatinglocket" | "innervating locket" | "innervating_locket" | "innervatinglocket" => ItemId::InnervatingLocket,"IONIAN BOOTS OF LUCIDITY" | "IONIANBOOTSOFLUCIDITY" | "IONIAN_BOOTS_OF_LUCIDITY" | "Ionian Boots of Lucidity" | "IonianBootsOfLucidity" | "Ionianbootsoflucidity" | "ionian boots of lucidity" | "ionian_boots_of_lucidity" | "ionianbootsoflucidity" => ItemId::IonianBootsOfLucidity,"JAK'SHO, THE PROTEAN" | "JAKSHOTHEPROTEAN" | "JAK_SHO_THE_PROTEAN" | "Jak'Sho, The Protean" | "JakShoTheProtean" | "Jakshotheprotean" | "jak'sho, the protean" | "jak_sho_the_protean" | "jakshotheprotean" => ItemId::JakShoTheProtean,"JARVAN I'S" | "JARVANIS" | "JARVAN_IS" | "Jarvan I's" | "JarvanIs" | "Jarvanis" | "jarvan i's" | "jarvan_is" | "jarvanis" => ItemId::JarvanIs,"JUICE OF HASTE" | "JUICEOFHASTE" | "JUICE_OF_HASTE" | "Juice of Haste" | "JuiceOfHaste" | "Juiceofhaste" | "juice of haste" | "juice_of_haste" | "juiceofhaste" => ItemId::JuiceOfHaste,"JUICE OF POWER" | "JUICEOFPOWER" | "JUICE_OF_POWER" | "Juice of Power" | "JuiceOfPower" | "Juiceofpower" | "juice of power" | "juice_of_power" | "juiceofpower" => ItemId::JuiceOfPower,"JUICE OF VITALITY" | "JUICEOFVITALITY" | "JUICE_OF_VITALITY" | "Juice of Vitality" | "JuiceOfVitality" | "Juiceofvitality" | "juice of vitality" | "juice_of_vitality" | "juiceofvitality" => ItemId::JuiceOfVitality,"KAENIC ROOKERN" | "KAENICROOKERN" | "KAENIC_ROOKERN" | "Kaenic Rookern" | "KaenicRookern" | "Kaenicrookern" | "kaenic rookern" | "kaenic_rookern" | "kaenicrookern" => ItemId::KaenicRookern,"KINDLEGEM" | "Kindlegem" | "kindlegem" => ItemId::Kindlegem,"KINKOU JITTE" | "KINKOUJITTE" | "KINKOU_JITTE" | "Kinkou Jitte" | "KinkouJitte" | "Kinkoujitte" | "kinkou jitte" | "kinkou_jitte" | "kinkoujitte" => ItemId::KinkouJitte,"KNIGHT'S VOW" | "KNIGHTSVOW" | "KNIGHTS_VOW" | "Knight's Vow" | "KnightsVow" | "Knightsvow" | "knight's vow" | "knights_vow" | "knightsvow" => ItemId::KnightsVow,"KRAKEN SLAYER" | "KRAKENSLAYER" | "KRAKEN_SLAYER" | "Kraken Slayer" | "KrakenSlayer" | "Krakenslayer" | "kraken slayer" | "kraken_slayer" | "krakenslayer" => ItemId::KrakenSlayer,"LAST WHISPER" | "LASTWHISPER" | "LAST_WHISPER" | "Last Whisper" | "LastWhisper" | "Lastwhisper" | "last whisper" | "last_whisper" | "lastwhisper" => ItemId::LastWhisper,"LEGENDARY ASSASSIN ITEM" | "LEGENDARYASSASSINITEM" | "LEGENDARY_ASSASSIN_ITEM" | "Legendary Assassin Item" | "LegendaryAssassinItem" | "Legendaryassassinitem" | "legendary assassin item" | "legendary_assassin_item" | "legendaryassassinitem" => ItemId::LegendaryAssassinItem,"LEGENDARY FIGHTER ITEM" | "LEGENDARYFIGHTERITEM" | "LEGENDARY_FIGHTER_ITEM" | "Legendary Fighter Item" | "LegendaryFighterItem" | "Legendaryfighteritem" | "legendary fighter item" | "legendary_fighter_item" | "legendaryfighteritem" => ItemId::LegendaryFighterItem,"LEGENDARY MAGE ITEM" | "LEGENDARYMAGEITEM" | "LEGENDARY_MAGE_ITEM" | "Legendary Mage Item" | "LegendaryMageItem" | "Legendarymageitem" | "legendary mage item" | "legendary_mage_item" | "legendarymageitem" => ItemId::LegendaryMageItem,"LEGENDARY MARKSMAN ITEM" | "LEGENDARYMARKSMANITEM" | "LEGENDARY_MARKSMAN_ITEM" | "Legendary Marksman Item" | "LegendaryMarksmanItem" | "Legendarymarksmanitem" | "legendary marksman item" | "legendary_marksman_item" | "legendarymarksmanitem" => ItemId::LegendaryMarksmanItem,"LEGENDARY SUPPORT ITEM" | "LEGENDARYSUPPORTITEM" | "LEGENDARY_SUPPORT_ITEM" | "Legendary Support Item" | "LegendarySupportItem" | "Legendarysupportitem" | "legendary support item" | "legendary_support_item" | "legendarysupportitem" => ItemId::LegendarySupportItem,"LEGENDARY TANK ITEM" | "LEGENDARYTANKITEM" | "LEGENDARY_TANK_ITEM" | "Legendary Tank Item" | "LegendaryTankItem" | "Legendarytankitem" | "legendary tank item" | "legendary_tank_item" | "legendarytankitem" => ItemId::LegendaryTankItem,"LIANDRY'S TORMENT" | "LIANDRYSTORMENT" | "LIANDRYS_TORMENT" | "Liandry's Torment" | "LiandrysTorment" | "Liandrystorment" | "liandry's torment" | "liandrys_torment" | "liandrystorment" => ItemId::LiandrysTorment,"LICH BANE" | "LICHBANE" | "LICH_BANE" | "Lich Bane" | "LichBane" | "Lichbane" | "lich bane" | "lich_bane" | "lichbane" => ItemId::LichBane,"LIFELINE" | "Lifeline" | "lifeline" => ItemId::Lifeline,"LIGHTNING ROD" | "LIGHTNINGROD" | "LIGHTNING_ROD" | "Lightning Rod" | "LightningRod" | "Lightningrod" | "lightning rod" | "lightning_rod" | "lightningrod" => ItemId::LightningRod,"LOCKET OF THE IRON SOLARI" | "LOCKETOFTHEIRONSOLARI" | "LOCKET_OF_THE_IRON_SOLARI" | "Locket of the Iron Solari" | "LocketOfTheIronSolari" | "Locketoftheironsolari" | "locket of the iron solari" | "locket_of_the_iron_solari" | "locketoftheironsolari" => ItemId::LocketOfTheIronSolari,"LONG SWORD" | "LONGSWORD" | "LONG_SWORD" | "Long Sword" | "LongSword" | "Longsword" | "long sword" | "long_sword" | "longsword" => ItemId::LongSword,"LORD DOMINIK'S REGARDS" | "LORDDOMINIKSREGARDS" | "LORD_DOMINIKS_REGARDS" | "Lord Dominik's Regards" | "LordDominiksRegards" | "Lorddominiksregards" | "lord dominik's regards" | "lord_dominiks_regards" | "lorddominiksregards" => ItemId::LordDominiksRegards,"LOST CHAPTER" | "LOSTCHAPTER" | "LOST_CHAPTER" | "Lost Chapter" | "LostChapter" | "Lostchapter" | "lost chapter" | "lost_chapter" | "lostchapter" => ItemId::LostChapter,"LUCKY DICE" | "LUCKYDICE" | "LUCKY_DICE" | "Lucky Dice" | "LuckyDice" | "Luckydice" | "lucky dice" | "lucky_dice" | "luckydice" => ItemId::LuckyDice,"LUDEN'S ECHO" | "LUDENSECHO" | "LUDENS_ECHO" | "Luden's Echo" | "LudensEcho" | "Ludensecho" | "luden's echo" | "ludens_echo" | "ludensecho" => ItemId::LudensEcho,"MALIGNANCE" | "Malignance" | "malignance" => ItemId::Malignance,"MANAMUNE" | "Manamune" | "manamune" => ItemId::Manamune,"MAW OF MALMORTIUS" | "MAWOFMALMORTIUS" | "MAW_OF_MALMORTIUS" | "Maw of Malmortius" | "MawOfMalmortius" | "Mawofmalmortius" | "maw of malmortius" | "maw_of_malmortius" | "mawofmalmortius" => ItemId::MawOfMalmortius,"MEJAI'S SOULSTEALER" | "MEJAISSOULSTEALER" | "MEJAIS_SOULSTEALER" | "Mejai's Soulstealer" | "MejaisSoulstealer" | "Mejaissoulstealer" | "mejai's soulstealer" | "mejais_soulstealer" | "mejaissoulstealer" => ItemId::MejaisSoulstealer,"MERCURIAL SCIMITAR" | "MERCURIALSCIMITAR" | "MERCURIAL_SCIMITAR" | "Mercurial Scimitar" | "MercurialScimitar" | "Mercurialscimitar" | "mercurial scimitar" | "mercurial_scimitar" | "mercurialscimitar" => ItemId::MercurialScimitar,"MERCURY'S TREADS" | "MERCURYSTREADS" | "MERCURYS_TREADS" | "Mercury's Treads" | "MercurysTreads" | "Mercurystreads" | "mercury's treads" | "mercurys_treads" | "mercurystreads" => ItemId::MercurysTreads,"MIKAEL'S BLESSING" | "MIKAELSBLESSING" | "MIKAELS_BLESSING" | "Mikael's Blessing" | "MikaelsBlessing" | "Mikaelsblessing" | "mikael's blessing" | "mikaels_blessing" | "mikaelsblessing" => ItemId::MikaelsBlessing,"MIRAGE BLADE" | "MIRAGEBLADE" | "MIRAGE_BLADE" | "Mirage Blade" | "MirageBlade" | "Mirageblade" | "mirage blade" | "mirage_blade" | "mirageblade" => ItemId::MirageBlade,"MOONFLAIR SPELLBLADE" | "MOONFLAIRSPELLBLADE" | "MOONFLAIR_SPELLBLADE" | "Moonflair Spellblade" | "MoonflairSpellblade" | "Moonflairspellblade" | "moonflair spellblade" | "moonflair_spellblade" | "moonflairspellblade" => ItemId::MoonflairSpellblade,"MOONSTONE RENEWER" | "MOONSTONERENEWER" | "MOONSTONE_RENEWER" | "Moonstone Renewer" | "MoonstoneRenewer" | "Moonstonerenewer" | "moonstone renewer" | "moonstone_renewer" | "moonstonerenewer" => ItemId::MoonstoneRenewer,"MORELLONOMICON" | "Morellonomicon" | "morellonomicon" => ItemId::Morellonomicon,"MORTAL REMINDER" | "MORTALREMINDER" | "MORTAL_REMINDER" | "Mortal Reminder" | "MortalReminder" | "Mortalreminder" | "mortal reminder" | "mortal_reminder" | "mortalreminder" => ItemId::MortalReminder,"MOSSTOMPER SEEDLING" | "MOSSTOMPERSEEDLING" | "MOSSTOMPER_SEEDLING" | "Mosstomper Seedling" | "MosstomperSeedling" | "Mosstomperseedling" | "mosstomper seedling" | "mosstomper_seedling" | "mosstomperseedling" => ItemId::MosstomperSeedling,"MULTITOOL" | "Multitool" | "multitool" => ItemId::Multitool,"MURAMANA" | "Muramana" | "muramana" => ItemId::Muramana,"NASHOR'S TOOTH" | "NASHORSTOOTH" | "NASHORS_TOOTH" | "Nashor's Tooth" | "NashorsTooth" | "Nashorstooth" | "nashor's tooth" | "nashors_tooth" | "nashorstooth" => ItemId::NashorsTooth,"NAVORI FLICKERBLADE" | "NAVORIFLICKERBLADE" | "NAVORI_FLICKERBLADE" | "Navori Flickerblade" | "NavoriFlickerblade" | "Navoriflickerblade" | "navori flickerblade" | "navori_flickerblade" | "navoriflickerblade" => ItemId::NavoriFlickerblade,"NEEDLESSLY LARGE ROD" | "NEEDLESSLYLARGEROD" | "NEEDLESSLY_LARGE_ROD" | "Needlessly Large Rod" | "NeedlesslyLargeRod" | "Needlesslylargerod" | "needlessly large rod" | "needlessly_large_rod" | "needlesslylargerod" => ItemId::NeedlesslyLargeRod,"NEGATRON CLOAK" | "NEGATRONCLOAK" | "NEGATRON_CLOAK" | "Negatron Cloak" | "NegatronCloak" | "Negatroncloak" | "negatron cloak" | "negatron_cloak" | "negatroncloak" => ItemId::NegatronCloak,"NIGHT HARVESTER" | "NIGHTHARVESTER" | "NIGHT_HARVESTER" | "Night Harvester" | "NightHarvester" | "Nightharvester" | "night harvester" | "night_harvester" | "nightharvester" => ItemId::NightHarvester,"NOONQUIVER" | "Noonquiver" | "noonquiver" => ItemId::Noonquiver,"NULL-MAGIC MANTLE" | "NULLMAGICMANTLE" | "NULL_MAGIC_MANTLE" | "Null-Magic Mantle" | "NullMagicMantle" | "Nullmagicmantle" | "null-magic mantle" | "null_magic_mantle" | "nullmagicmantle" => ItemId::NullMagicMantle,"OBLIVION ORB" | "OBLIVIONORB" | "OBLIVION_ORB" | "Oblivion Orb" | "OblivionOrb" | "Oblivionorb" | "oblivion orb" | "oblivion_orb" | "oblivionorb" => ItemId::OblivionOrb,"OHMWRECKER (TURRET ITEM)" | "OHMWRECKERTURRETITEM" | "OHMWRECKER_TURRET_ITEM" | "Ohmwrecker (Turret Item)" | "OhmwreckerTurretItem" | "Ohmwreckerturretitem" | "ohmwrecker (turret item)" | "ohmwrecker_turret_item" | "ohmwreckerturretitem" => ItemId::OhmwreckerTurretItem,"ORACLE LENS" | "ORACLELENS" | "ORACLE_LENS" | "Oracle Lens" | "OracleLens" | "Oraclelens" | "oracle lens" | "oracle_lens" | "oraclelens" => ItemId::OracleLens,"OVERCHARGED" | "Overcharged" | "overcharged" => ItemId::Overcharged,"OVERLORD'S BLOODMAIL" | "OVERLORDSBLOODMAIL" | "OVERLORDS_BLOODMAIL" | "Overlord's Bloodmail" | "OverlordsBloodmail" | "Overlordsbloodmail" | "overlord's bloodmail" | "overlords_bloodmail" | "overlordsbloodmail" => ItemId::OverlordsBloodmail,"PERPLEXITY" | "Perplexity" | "perplexity" => ItemId::Perplexity,"PHAGE" | "Phage" | "phage" => ItemId::Phage,"PHANTOM DANCER" | "PHANTOMDANCER" | "PHANTOM_DANCER" | "Phantom Dancer" | "PhantomDancer" | "Phantomdancer" | "phantom dancer" | "phantom_dancer" | "phantomdancer" => ItemId::PhantomDancer,"PHREAKISH GUSTO" | "PHREAKISHGUSTO" | "PHREAKISH_GUSTO" | "Phreakish Gusto" | "PhreakishGusto" | "Phreakishgusto" | "phreakish gusto" | "phreakish_gusto" | "phreakishgusto" => ItemId::PhreakishGusto,"PICKAXE" | "Pickaxe" | "pickaxe" => ItemId::Pickaxe,"PLATED STEELCAPS" | "PLATEDSTEELCAPS" | "PLATED_STEELCAPS" | "Plated Steelcaps" | "PlatedSteelcaps" | "Platedsteelcaps" | "plated steelcaps" | "plated_steelcaps" | "platedsteelcaps" => ItemId::PlatedSteelcaps,"PORO-SNAX" | "POROSNAX" | "PORO_SNAX" | "Poro-Snax" | "PoroSnax" | "Porosnax" | "poro-snax" | "poro_snax" | "porosnax" => ItemId::PoroSnax,"PRISMATIC ITEM" | "PRISMATICITEM" | "PRISMATIC_ITEM" | "Prismatic Item" | "PrismaticItem" | "Prismaticitem" | "prismatic item" | "prismatic_item" | "prismaticitem" => ItemId::PrismaticItem,"PRISMATIC STAT VOUCHER" | "PRISMATICSTATVOUCHER" | "PRISMATIC_STAT_VOUCHER" | "Prismatic Stat Voucher" | "PrismaticStatVoucher" | "Prismaticstatvoucher" | "prismatic stat voucher" | "prismatic_stat_voucher" | "prismaticstatvoucher" => ItemId::PrismaticStatVoucher,"PROFANE HYDRA" | "PROFANEHYDRA" | "PROFANE_HYDRA" | "Profane Hydra" | "ProfaneHydra" | "Profanehydra" | "profane hydra" | "profane_hydra" | "profanehydra" => ItemId::ProfaneHydra,"PROTOPLASM HARNESS" | "PROTOPLASMHARNESS" | "PROTOPLASM_HARNESS" | "Protoplasm Harness" | "ProtoplasmHarness" | "Protoplasmharness" | "protoplasm harness" | "protoplasm_harness" | "protoplasmharness" => ItemId::ProtoplasmHarness,"PROWLER'S CLAW" | "PROWLERSCLAW" | "PROWLERS_CLAW" | "Prowler's Claw" | "ProwlersClaw" | "Prowlersclaw" | "prowler's claw" | "prowlers_claw" | "prowlersclaw" => ItemId::ProwlersClaw,"PUPPETEER" | "Puppeteer" | "puppeteer" => ItemId::Puppeteer,"PYROMANCER'S CLOAK" | "PYROMANCERSCLOAK" | "PYROMANCERS_CLOAK" | "Pyromancer's Cloak" | "PyromancersCloak" | "Pyromancerscloak" | "pyromancer's cloak" | "pyromancers_cloak" | "pyromancerscloak" => ItemId::PyromancersCloak,"QUICKSILVER SASH" | "QUICKSILVERSASH" | "QUICKSILVER_SASH" | "Quicksilver Sash" | "QuicksilverSash" | "Quicksilversash" | "quicksilver sash" | "quicksilver_sash" | "quicksilversash" => ItemId::QuicksilverSash,"RABADON'S DEATHCAP" | "RABADONSDEATHCAP" | "RABADONS_DEATHCAP" | "Rabadon's Deathcap" | "RabadonsDeathcap" | "Rabadonsdeathcap" | "rabadon's deathcap" | "rabadons_deathcap" | "rabadonsdeathcap" => ItemId::RabadonsDeathcap,"RADIANT VIRTUE" | "RADIANTVIRTUE" | "RADIANT_VIRTUE" | "Radiant Virtue" | "RadiantVirtue" | "Radiantvirtue" | "radiant virtue" | "radiant_virtue" | "radiantvirtue" => ItemId::RadiantVirtue,"RAISE MORALE" | "RAISEMORALE" | "RAISE_MORALE" | "Raise Morale" | "RaiseMorale" | "Raisemorale" | "raise morale" | "raise_morale" | "raisemorale" => ItemId::RaiseMorale,"RANDUIN'S OMEN" | "RANDUINSOMEN" | "RANDUINS_OMEN" | "Randuin's Omen" | "RanduinsOmen" | "Randuinsomen" | "randuin's omen" | "randuins_omen" | "randuinsomen" => ItemId::RanduinsOmen,"RAPID FIRECANNON" | "RAPIDFIRECANNON" | "RAPID_FIRECANNON" | "Rapid Firecannon" | "RapidFirecannon" | "Rapidfirecannon" | "rapid firecannon" | "rapid_firecannon" | "rapidfirecannon" => ItemId::RapidFirecannon,"RAVENOUS HYDRA" | "RAVENOUSHYDRA" | "RAVENOUS_HYDRA" | "Ravenous Hydra" | "RavenousHydra" | "Ravenoushydra" | "ravenous hydra" | "ravenous_hydra" | "ravenoushydra" => ItemId::RavenousHydra,"REALITY FRACTURE" | "REALITYFRACTURE" | "REALITY_FRACTURE" | "Reality Fracture" | "RealityFracture" | "Realityfracture" | "reality fracture" | "reality_fracture" | "realityfracture" => ItemId::RealityFracture,"REAPER'S TOLL" | "REAPERSTOLL" | "REAPERS_TOLL" | "Reaper's Toll" | "ReapersToll" | "Reaperstoll" | "reaper's toll" | "reapers_toll" | "reaperstoll" => ItemId::ReapersToll,"RECTRIX" | "Rectrix" | "rectrix" => ItemId::Rectrix,"RECURVE BOW" | "RECURVEBOW" | "RECURVE_BOW" | "Recurve Bow" | "RecurveBow" | "Recurvebow" | "recurve bow" | "recurve_bow" | "recurvebow" => ItemId::RecurveBow,"REDEMPTION" | "Redemption" | "redemption" => ItemId::Redemption,"REFILLABLE POTION" | "REFILLABLEPOTION" | "REFILLABLE_POTION" | "Refillable Potion" | "RefillablePotion" | "Refillablepotion" | "refillable potion" | "refillable_potion" | "refillablepotion" => ItemId::RefillablePotion,"REGICIDE" | "Regicide" | "regicide" => ItemId::Regicide,"REINFORCED ARMOR (TURRET ITEM)" | "REINFORCEDARMORTURRETITEM" | "REINFORCED_ARMOR_TURRET_ITEM" | "Reinforced Armor (Turret Item)" | "ReinforcedArmorTurretItem" | "Reinforcedarmorturretitem" | "reinforced armor (turret item)" | "reinforced_armor_turret_item" | "reinforcedarmorturretitem" => ItemId::ReinforcedArmorTurretItem,"REJUVENATION BEAD" | "REJUVENATIONBEAD" | "REJUVENATION_BEAD" | "Rejuvenation Bead" | "RejuvenationBead" | "Rejuvenationbead" | "rejuvenation bead" | "rejuvenation_bead" | "rejuvenationbead" => ItemId::RejuvenationBead,"REVERBERATION" | "Reverberation" | "reverberation" => ItemId::Reverberation,"RIFTMAKER" | "Riftmaker" | "riftmaker" => ItemId::Riftmaker,"RITE OF RUIN" | "RITEOFRUIN" | "RITE_OF_RUIN" | "Rite of Ruin" | "RiteOfRuin" | "Riteofruin" | "rite of ruin" | "rite_of_ruin" | "riteofruin" => ItemId::RiteOfRuin,"ROD OF AGES" | "RODOFAGES" | "ROD_OF_AGES" | "Rod of Ages" | "RodOfAges" | "Rodofages" | "rod of ages" | "rod_of_ages" | "rodofages" => ItemId::RodOfAges,"RUBY CRYSTAL" | "RUBYCRYSTAL" | "RUBY_CRYSTAL" | "Ruby Crystal" | "RubyCrystal" | "Rubycrystal" | "ruby crystal" | "ruby_crystal" | "rubycrystal" => ItemId::RubyCrystal,"RUNAAN'S HURRICANE" | "RUNAANSHURRICANE" | "RUNAANS_HURRICANE" | "Runaan's Hurricane" | "RunaansHurricane" | "Runaanshurricane" | "runaan's hurricane" | "runaans_hurricane" | "runaanshurricane" => ItemId::RunaansHurricane,"RUNECARVER" | "Runecarver" | "runecarver" => ItemId::Runecarver,"RUNIC COMPASS" | "RUNICCOMPASS" | "RUNIC_COMPASS" | "Runic Compass" | "RunicCompass" | "Runiccompass" | "runic compass" | "runic_compass" | "runiccompass" => ItemId::RunicCompass,"RYLAI'S CRYSTAL SCEPTER" | "RYLAISCRYSTALSCEPTER" | "RYLAIS_CRYSTAL_SCEPTER" | "Rylai's Crystal Scepter" | "RylaisCrystalScepter" | "Rylaiscrystalscepter" | "rylai's crystal scepter" | "rylais_crystal_scepter" | "rylaiscrystalscepter" => ItemId::RylaisCrystalScepter,"SANGUINE GIFT" | "SANGUINEGIFT" | "SANGUINE_GIFT" | "Sanguine Gift" | "SanguineGift" | "Sanguinegift" | "sanguine gift" | "sanguine_gift" | "sanguinegift" => ItemId::SanguineGift,"SAPPHIRE CRYSTAL" | "SAPPHIRECRYSTAL" | "SAPPHIRE_CRYSTAL" | "Sapphire Crystal" | "SapphireCrystal" | "Sapphirecrystal" | "sapphire crystal" | "sapphire_crystal" | "sapphirecrystal" => ItemId::SapphireCrystal,"SCARECROW EFFIGY" | "SCARECROWEFFIGY" | "SCARECROW_EFFIGY" | "Scarecrow Effigy" | "ScarecrowEffigy" | "Scarecroweffigy" | "scarecrow effigy" | "scarecrow_effigy" | "scarecroweffigy" => ItemId::ScarecrowEffigy,"SCORCHCLAW PUP" | "SCORCHCLAWPUP" | "SCORCHCLAW_PUP" | "Scorchclaw Pup" | "ScorchclawPup" | "Scorchclawpup" | "scorchclaw pup" | "scorchclaw_pup" | "scorchclawpup" => ItemId::ScorchclawPup,"SCOUTING AHEAD" | "SCOUTINGAHEAD" | "SCOUTING_AHEAD" | "Scouting Ahead" | "ScoutingAhead" | "Scoutingahead" | "scouting ahead" | "scouting_ahead" | "scoutingahead" => ItemId::ScoutingAhead,"SCOUT'S SLINGSHOT" | "SCOUTSSLINGSHOT" | "SCOUTS_SLINGSHOT" | "Scout's Slingshot" | "ScoutsSlingshot" | "Scoutsslingshot" | "scout's slingshot" | "scouts_slingshot" | "scoutsslingshot" => ItemId::ScoutsSlingshot,"SEEKER'S ARMGUARD" | "SEEKERSARMGUARD" | "SEEKERS_ARMGUARD" | "Seeker's Armguard" | "SeekersArmguard" | "Seekersarmguard" | "seeker's armguard" | "seekers_armguard" | "seekersarmguard" => ItemId::SeekersArmguard,"SERAPH'S EMBRACE" | "SERAPHSEMBRACE" | "SERAPHS_EMBRACE" | "Seraph's Embrace" | "SeraphsEmbrace" | "Seraphsembrace" | "seraph's embrace" | "seraphs_embrace" | "seraphsembrace" => ItemId::SeraphsEmbrace,"SERPENT'S FANG" | "SERPENTSFANG" | "SERPENTS_FANG" | "Serpent's Fang" | "SerpentsFang" | "Serpentsfang" | "serpent's fang" | "serpents_fang" | "serpentsfang" => ItemId::SerpentsFang,"SERRATED DIRK" | "SERRATEDDIRK" | "SERRATED_DIRK" | "Serrated Dirk" | "SerratedDirk" | "Serrateddirk" | "serrated dirk" | "serrated_dirk" | "serrateddirk" => ItemId::SerratedDirk,"SERYLDA'S GRUDGE" | "SERYLDASGRUDGE" | "SERYLDAS_GRUDGE" | "Serylda's Grudge" | "SeryldasGrudge" | "Seryldasgrudge" | "serylda's grudge" | "seryldas_grudge" | "seryldasgrudge" => ItemId::SeryldasGrudge,"SHADOWFLAME" | "Shadowflame" | "shadowflame" => ItemId::Shadowflame,"SHATTERED ARMGUARD" | "SHATTEREDARMGUARD" | "SHATTERED_ARMGUARD" | "Shattered Armguard" | "ShatteredArmguard" | "Shatteredarmguard" | "shattered armguard" | "shattered_armguard" | "shatteredarmguard" => ItemId::ShatteredArmguard,"SHEEN" | "Sheen" | "sheen" => ItemId::Sheen,"SHIELD OF MOLTEN STONE" | "SHIELDOFMOLTENSTONE" | "SHIELD_OF_MOLTEN_STONE" | "Shield of Molten Stone" | "ShieldOfMoltenStone" | "Shieldofmoltenstone" | "shield of molten stone" | "shield_of_molten_stone" | "shieldofmoltenstone" => ItemId::ShieldOfMoltenStone,"SHURELYA'S BATTLESONG" | "SHURELYASBATTLESONG" | "SHURELYAS_BATTLESONG" | "Shurelya's Battlesong" | "ShurelyasBattlesong" | "Shurelyasbattlesong" | "shurelya's battlesong" | "shurelyas_battlesong" | "shurelyasbattlesong" => ItemId::ShurelyasBattlesong,"SLIGHTLY MAGICAL BOOTS" | "SLIGHTLYMAGICALBOOTS" | "SLIGHTLY_MAGICAL_BOOTS" | "Slightly Magical Boots" | "SlightlyMagicalBoots" | "Slightlymagicalboots" | "slightly magical boots" | "slightly_magical_boots" | "slightlymagicalboots" => ItemId::SlightlyMagicalBoots,"SOLSTICE SLEIGH" | "SOLSTICESLEIGH" | "SOLSTICE_SLEIGH" | "Solstice Sleigh" | "SolsticeSleigh" | "Solsticesleigh" | "solstice sleigh" | "solstice_sleigh" | "solsticesleigh" => ItemId::SolsticeSleigh,"SORCERER'S SHOES" | "SORCERERSSHOES" | "SORCERERS_SHOES" | "Sorcerer's Shoes" | "SorcerersShoes" | "Sorcerersshoes" | "sorcerer's shoes" | "sorcerers_shoes" | "sorcerersshoes" => ItemId::SorcerersShoes,"SPEAR OF SHOJIN" | "SPEAROFSHOJIN" | "SPEAR_OF_SHOJIN" | "Spear of Shojin" | "SpearOfShojin" | "Spearofshojin" | "spear of shojin" | "spear_of_shojin" | "spearofshojin" => ItemId::SpearOfShojin,"SPECTRAL CUTLASS" | "SPECTRALCUTLASS" | "SPECTRAL_CUTLASS" | "Spectral Cutlass" | "SpectralCutlass" | "Spectralcutlass" | "spectral cutlass" | "spectral_cutlass" | "spectralcutlass" => ItemId::SpectralCutlass,"SPECTRE'S COWL" | "SPECTRESCOWL" | "SPECTRES_COWL" | "Spectre's Cowl" | "SpectresCowl" | "Spectrescowl" | "spectre's cowl" | "spectres_cowl" | "spectrescowl" => ItemId::SpectresCowl,"SPELLSLINGER'S SHOES" | "SPELLSLINGERSSHOES" | "SPELLSLINGERS_SHOES" | "Spellslinger's Shoes" | "SpellslingersShoes" | "Spellslingersshoes" | "spellslinger's shoes" | "spellslingers_shoes" | "spellslingersshoes" => ItemId::SpellslingersShoes,"SPIRIT VISAGE" | "SPIRITVISAGE" | "SPIRIT_VISAGE" | "Spirit Visage" | "SpiritVisage" | "Spiritvisage" | "spirit visage" | "spirit_visage" | "spiritvisage" => ItemId::SpiritVisage,"STAFF OF FLOWING WATER" | "STAFFOFFLOWINGWATER" | "STAFF_OF_FLOWING_WATER" | "Staff of Flowing Water" | "StaffOfFlowingWater" | "Staffofflowingwater" | "staff of flowing water" | "staff_of_flowing_water" | "staffofflowingwater" => ItemId::StaffOfFlowingWater,"STAT BONUS (ARAM: MAYHEM)" | "STATBONUSARAMMAYHEM" | "STAT_BONUS_ARAM_MAYHEM" | "Stat Bonus (ARAM: Mayhem)" | "StatBonusAramMayhem" | "Statbonusarammayhem" | "stat bonus (aram: mayhem)" | "stat_bonus_aram_mayhem" | "statbonusarammayhem" => ItemId::StatBonusAramMayhem,"STAT BONUS (ARENA)" | "STATBONUSARENA" | "STAT_BONUS_ARENA" | "Stat Bonus (Arena)" | "StatBonusArena" | "Statbonusarena" | "stat bonus (arena)" | "stat_bonus_arena" | "statbonusarena" => ItemId::StatBonusArena,"STATIKK SHIV" | "STATIKKSHIV" | "STATIKK_SHIV" | "Statikk Shiv" | "StatikkShiv" | "Statikkshiv" | "statikk shiv" | "statikk_shiv" | "statikkshiv" => ItemId::StatikkShiv,"STEALTH WARD" | "STEALTHWARD" | "STEALTH_WARD" | "Stealth Ward" | "StealthWard" | "Stealthward" | "stealth ward" | "stealth_ward" | "stealthward" => ItemId::StealthWard,"STEEL SIGIL" | "STEELSIGIL" | "STEEL_SIGIL" | "Steel Sigil" | "SteelSigil" | "Steelsigil" | "steel sigil" | "steel_sigil" | "steelsigil" => ItemId::SteelSigil,"STERAK'S GAGE" | "STERAKSGAGE" | "STERAKS_GAGE" | "Sterak's Gage" | "SteraksGage" | "Steraksgage" | "sterak's gage" | "steraks_gage" | "steraksgage" => ItemId::SteraksGage,"STORMRAZOR" | "Stormrazor" | "stormrazor" => ItemId::Stormrazor,"STORMSURGE" | "Stormsurge" | "stormsurge" => ItemId::Stormsurge,"STRIDEBREAKER" | "Stridebreaker" | "stridebreaker" => ItemId::Stridebreaker,"SUNDERED SKY" | "SUNDEREDSKY" | "SUNDERED_SKY" | "Sundered Sky" | "SunderedSky" | "Sunderedsky" | "sundered sky" | "sundered_sky" | "sunderedsky" => ItemId::SunderedSky,"SUNFIRE AEGIS" | "SUNFIREAEGIS" | "SUNFIRE_AEGIS" | "Sunfire Aegis" | "SunfireAegis" | "Sunfireaegis" | "sunfire aegis" | "sunfire_aegis" | "sunfireaegis" => ItemId::SunfireAegis,"SUPER MECH ARMOR" | "SUPERMECHARMOR" | "SUPER_MECH_ARMOR" | "Super Mech Armor" | "SuperMechArmor" | "Supermecharmor" | "super mech armor" | "super_mech_armor" | "supermecharmor" => ItemId::SuperMechArmor,"SUPER MECH POWER FIELD" | "SUPERMECHPOWERFIELD" | "SUPER_MECH_POWER_FIELD" | "Super Mech Power Field" | "SuperMechPowerField" | "Supermechpowerfield" | "super mech power field" | "super_mech_power_field" | "supermechpowerfield" => ItemId::SuperMechPowerField,"SWIFTMARCH" | "Swiftmarch" | "swiftmarch" => ItemId::Swiftmarch,"SWORD OF BLOSSOMING DAWN" | "SWORDOFBLOSSOMINGDAWN" | "SWORD_OF_BLOSSOMING_DAWN" | "Sword of Blossoming Dawn" | "SwordOfBlossomingDawn" | "Swordofblossomingdawn" | "sword of blossoming dawn" | "sword_of_blossoming_dawn" | "swordofblossomingdawn" => ItemId::SwordOfBlossomingDawn,"SWORD OF THE DIVINE" | "SWORDOFTHEDIVINE" | "SWORD_OF_THE_DIVINE" | "Sword of the Divine" | "SwordOfTheDivine" | "Swordofthedivine" | "sword of the divine" | "sword_of_the_divine" | "swordofthedivine" => ItemId::SwordOfTheDivine,"TALISMAN OF ASCENSION" | "TALISMANOFASCENSION" | "TALISMAN_OF_ASCENSION" | "Talisman of Ascension" | "TalismanOfAscension" | "Talismanofascension" | "talisman of ascension" | "talisman_of_ascension" | "talismanofascension" => ItemId::TalismanOfAscension,"TEAR OF THE GODDESS" | "TEAROFTHEGODDESS" | "TEAR_OF_THE_GODDESS" | "Tear of the Goddess" | "TearOfTheGoddess" | "Tearofthegoddess" | "tear of the goddess" | "tear_of_the_goddess" | "tearofthegoddess" => ItemId::TearOfTheGoddess,"TERMINUS" | "Terminus" | "terminus" => ItemId::Terminus,"THE BRUTALIZER" | "THEBRUTALIZER" | "THE_BRUTALIZER" | "The Brutalizer" | "TheBrutalizer" | "Thebrutalizer" | "the brutalizer" | "the_brutalizer" | "thebrutalizer" => ItemId::TheBrutalizer,"THE COLLECTOR" | "THECOLLECTOR" | "THE_COLLECTOR" | "The Collector" | "TheCollector" | "Thecollector" | "the collector" | "the_collector" | "thecollector" => ItemId::TheCollector,"THE GOLDEN SPATULA" | "THEGOLDENSPATULA" | "THE_GOLDEN_SPATULA" | "The Golden Spatula" | "TheGoldenSpatula" | "Thegoldenspatula" | "the golden spatula" | "the_golden_spatula" | "thegoldenspatula" => ItemId::TheGoldenSpatula,"THORNMAIL" | "Thornmail" | "thornmail" => ItemId::Thornmail,"TIAMAT" | "Tiamat" | "tiamat" => ItemId::Tiamat,"TITANIC HYDRA" | "TITANICHYDRA" | "TITANIC_HYDRA" | "Titanic Hydra" | "TitanicHydra" | "Titanichydra" | "titanic hydra" | "titanic_hydra" | "titanichydra" => ItemId::TitanicHydra,"TOTAL BISCUIT OF EVERLASTING WILL" | "TOTALBISCUITOFEVERLASTINGWILL" | "TOTAL_BISCUIT_OF_EVERLASTING_WILL" | "Total Biscuit of Everlasting Will" | "TotalBiscuitOfEverlastingWill" | "Totalbiscuitofeverlastingwill" | "total biscuit of everlasting will" | "total_biscuit_of_everlasting_will" | "totalbiscuitofeverlastingwill" => ItemId::TotalBiscuitOfEverlastingWill,"TOWER POWER-UP" | "TOWERPOWERUP" | "TOWER_POWER_UP" | "Tower Power-Up" | "TowerPowerUp" | "Towerpowerup" | "tower power-up" | "tower_power_up" | "towerpowerup" => ItemId::TowerPowerUp,"TRINITY FORCE" | "TRINITYFORCE" | "TRINITY_FORCE" | "Trinity Force" | "TrinityForce" | "Trinityforce" | "trinity force" | "trinity_force" | "trinityforce" => ItemId::TrinityForce,"TUNNELER" | "Tunneler" | "tunneler" => ItemId::Tunneler,"TURBO CHEMTANK" | "TURBOCHEMTANK" | "TURBO_CHEMTANK" | "Turbo Chemtank" | "TurboChemtank" | "Turbochemtank" | "turbo chemtank" | "turbo_chemtank" | "turbochemtank" => ItemId::TurboChemtank,"TURRET PLATING" | "TURRETPLATING" | "TURRET_PLATING" | "Turret Plating" | "TurretPlating" | "Turretplating" | "turret plating" | "turret_plating" | "turretplating" => ItemId::TurretPlating,"TWILIGHT'S EDGE" | "TWILIGHTSEDGE" | "TWILIGHTS_EDGE" | "Twilight's Edge" | "TwilightsEdge" | "Twilightsedge" | "twilight's edge" | "twilights_edge" | "twilightsedge" => ItemId::TwilightsEdge,"TWIN MASK" | "TWINMASK" | "TWIN_MASK" | "Twin Mask" | "TwinMask" | "Twinmask" | "twin mask" | "twin_mask" | "twinmask" => ItemId::TwinMask,"UMBRAL GLAIVE" | "UMBRALGLAIVE" | "UMBRAL_GLAIVE" | "Umbral Glaive" | "UmbralGlaive" | "Umbralglaive" | "umbral glaive" | "umbral_glaive" | "umbralglaive" => ItemId::UmbralGlaive,"UNENDING DESPAIR" | "UNENDINGDESPAIR" | "UNENDING_DESPAIR" | "Unending Despair" | "UnendingDespair" | "Unendingdespair" | "unending despair" | "unending_despair" | "unendingdespair" => ItemId::UnendingDespair,"VAMPIRIC SCEPTER" | "VAMPIRICSCEPTER" | "VAMPIRIC_SCEPTER" | "Vampiric Scepter" | "VampiricScepter" | "Vampiricscepter" | "vampiric scepter" | "vampiric_scepter" | "vampiricscepter" => ItemId::VampiricScepter,"VEIGAR'S TALISMAN OF ASCENSION" | "VEIGARSTALISMANOFASCENSION" | "VEIGARS_TALISMAN_OF_ASCENSION" | "Veigar's Talisman of Ascension" | "VeigarsTalismanOfAscension" | "Veigarstalismanofascension" | "veigar's talisman of ascension" | "veigars_talisman_of_ascension" | "veigarstalismanofascension" => ItemId::VeigarsTalismanOfAscension,"VERDANT BARRIER" | "VERDANTBARRIER" | "VERDANT_BARRIER" | "Verdant Barrier" | "VerdantBarrier" | "Verdantbarrier" | "verdant barrier" | "verdant_barrier" | "verdantbarrier" => ItemId::VerdantBarrier,"VOID IMMOLATION" | "VOIDIMMOLATION" | "VOID_IMMOLATION" | "Void Immolation" | "VoidImmolation" | "Voidimmolation" | "void immolation" | "void_immolation" | "voidimmolation" => ItemId::VoidImmolation,"VOID STAFF" | "VOIDSTAFF" | "VOID_STAFF" | "Void Staff" | "VoidStaff" | "Voidstaff" | "void staff" | "void_staff" | "voidstaff" => ItemId::VoidStaff,"VOLTAIC CYCLOSWORD" | "VOLTAICCYCLOSWORD" | "VOLTAIC_CYCLOSWORD" | "Voltaic Cyclosword" | "VoltaicCyclosword" | "Voltaiccyclosword" | "voltaic cyclosword" | "voltaic_cyclosword" | "voltaiccyclosword" => ItemId::VoltaicCyclosword,"WARDEN'S EYE" | "WARDENSEYE" | "WARDENS_EYE" | "Warden's Eye" | "WardensEye" | "Wardenseye" | "warden's eye" | "wardens_eye" | "wardenseye" => ItemId::WardensEye,"WARDEN'S MAIL" | "WARDENSMAIL" | "WARDENS_MAIL" | "Warden's Mail" | "WardensMail" | "Wardensmail" | "warden's mail" | "wardens_mail" | "wardensmail" => ItemId::WardensMail,"WARMOG'S ARMOR" | "WARMOGSARMOR" | "WARMOGS_ARMOR" | "Warmog's Armor" | "WarmogsArmor" | "Warmogsarmor" | "warmog's armor" | "warmogs_armor" | "warmogsarmor" => ItemId::WarmogsArmor,"WHISPERING CIRCLET" | "WHISPERINGCIRCLET" | "WHISPERING_CIRCLET" | "Whispering Circlet" | "WhisperingCirclet" | "Whisperingcirclet" | "whispering circlet" | "whispering_circlet" | "whisperingcirclet" => ItemId::WhisperingCirclet,"WINGED MOONPLATE" | "WINGEDMOONPLATE" | "WINGED_MOONPLATE" | "Winged Moonplate" | "WingedMoonplate" | "Wingedmoonplate" | "winged moonplate" | "winged_moonplate" | "wingedmoonplate" => ItemId::WingedMoonplate,"WINTER'S APPROACH" | "WINTERSAPPROACH" | "WINTERS_APPROACH" | "Winter's Approach" | "WintersApproach" | "Wintersapproach" | "winter's approach" | "winters_approach" | "wintersapproach" => ItemId::WintersApproach,"WIT'S END" | "WITSEND" | "WITS_END" | "Wit's End" | "WitsEnd" | "Witsend" | "wit's end" | "wits_end" | "witsend" => ItemId::WitsEnd,"WOOGLET'S WITCHCAP" | "WOOGLETSWITCHCAP" | "WOOGLETS_WITCHCAP" | "Wooglet's Witchcap" | "WoogletsWitchcap" | "Woogletswitchcap" | "wooglet's witchcap" | "wooglets_witchcap" | "woogletswitchcap" => ItemId::WoogletsWitchcap,"WORDLESS PROMISE" | "WORDLESSPROMISE" | "WORDLESS_PROMISE" | "Wordless Promise" | "WordlessPromise" | "Wordlesspromise" | "wordless promise" | "wordless_promise" | "wordlesspromise" => ItemId::WordlessPromise,"WORLD ATLAS" | "WORLDATLAS" | "WORLD_ATLAS" | "World Atlas" | "WorldAtlas" | "Worldatlas" | "world atlas" | "world_atlas" | "worldatlas" => ItemId::WorldAtlas,"YOUMUU'S GHOSTBLADE" | "YOUMUUSGHOSTBLADE" | "YOUMUUS_GHOSTBLADE" | "Youmuu's Ghostblade" | "YoumuusGhostblade" | "Youmuusghostblade" | "youmuu's ghostblade" | "youmuus_ghostblade" | "youmuusghostblade" => ItemId::YoumuusGhostblade,"YOUR CUT" | "YOURCUT" | "YOUR_CUT" | "Your Cut" | "YourCut" | "Yourcut" | "your cut" | "your_cut" | "yourcut" => ItemId::YourCut,"YUN TAL WILDARROWS" | "YUNTALWILDARROWS" | "YUN_TAL_WILDARROWS" | "Yun Tal Wildarrows" | "YunTalWildarrows" | "Yuntalwildarrows" | "yun tal wildarrows" | "yun_tal_wildarrows" | "yuntalwildarrows" => ItemId::YunTalWildarrows,"ZAZ'ZAK'S REALMSPIKE" | "ZAZZAKSREALMSPIKE" | "ZAZ_ZAKS_REALMSPIKE" | "Zaz'Zak's Realmspike" | "ZazZaksRealmspike" | "Zazzaksrealmspike" | "zaz'zak's realmspike" | "zaz_zaks_realmspike" | "zazzaksrealmspike" => ItemId::ZazZaksRealmspike,"ZEAL" | "Zeal" | "zeal" => ItemId::Zeal,"ZEKE'S CONVERGENCE" | "ZEKESCONVERGENCE" | "ZEKES_CONVERGENCE" | "Zeke's Convergence" | "ZekesConvergence" | "Zekesconvergence" | "zeke's convergence" | "zekes_convergence" | "zekesconvergence" => ItemId::ZekesConvergence,"ZEPHYR" | "Zephyr" | "zephyr" => ItemId::Zephyr,"ZHONYA'S HOURGLASS" | "ZHONYASHOURGLASS" | "ZHONYAS_HOURGLASS" | "Zhonya's Hourglass" | "ZhonyasHourglass" | "Zhonyashourglass" | "zhonya's hourglass" | "zhonyas_hourglass" | "zhonyashourglass" => ItemId::ZhonyasHourglass);
pub const fn item_const_eval(
    ctx: &Ctx,
    item_id: ItemId,
    attack_type: AttackType,
) -> [f32; 2] {
    match item_id {
        ItemId::AbyssalMask => match attack_type {
            Melee => [abyssal_mask_melee_min(&ctx), zero(&ctx)],
            Ranged => [abyssal_mask_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::Actualizer => match attack_type {
            Melee => [actualizer_melee_min(&ctx), zero(&ctx)],
            Ranged => [actualizer_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::AetherWisp => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::AmplifyingTome => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::AnathemasChains => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::AntiTowerSocks => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::AnvilVoucher => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ArcaneSweeperTrinket => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ArchangelsStaff => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ArdentCenser => match attack_type {
            Melee => [ardent_censer_melee_min(&ctx), zero(&ctx)],
            Ranged => [ardent_censer_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::ArmoredAdvance => match attack_type {
            Melee => [armored_advance_melee_min(&ctx), zero(&ctx)],
            Ranged => [armored_advance_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::AtmasReckoning => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::AxiomArc => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::BFSword => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::BamisCinder => match attack_type {
            Melee => [bamis_cinder_melee_min(&ctx), zero(&ctx)],
            Ranged => [bamis_cinder_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::BandleglassMirror => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Bandlepipes => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::BansheesVeil => match attack_type {
            Melee => [banshees_veil_melee_min(&ctx), zero(&ctx)],
            Ranged => [banshees_veil_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::BaseTurretReinforcedArmorTurretItem => match attack_type {
            Melee => [
                base_turret_reinforced_armor_turret_item_melee_min(&ctx),
                zero(&ctx),
            ],
            Ranged => [
                base_turret_reinforced_armor_turret_item_melee_min(&ctx),
                zero(&ctx),
            ],
        },

        ItemId::Bastionbreaker => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::BerserkersGreaves => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::BlackCleaver => match attack_type {
            Melee => [black_cleaver_melee_min(&ctx), zero(&ctx)],
            Ranged => [black_cleaver_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::BlackHoleGauntlet => match attack_type {
            Melee => [black_hole_gauntlet_melee_min(&ctx), zero(&ctx)],
            Ranged => [black_hole_gauntlet_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::BlackSpear => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::BlackfireTorch => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::BladeOfTheRuinedKing => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::BlastingWand => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::BlightingJewel => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::BloodlettersCurse => match attack_type {
            Melee => [bloodletters_curse_melee_min(&ctx), zero(&ctx)],
            Ranged => [bloodletters_curse_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::Bloodsong => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Bloodthirster => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Boots => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::BootsOfSwiftness => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::BountyOfWorlds => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::BrambleVest => match attack_type {
            Melee => [bramble_vest_melee_min(&ctx), zero(&ctx)],
            Ranged => [bramble_vest_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::BraveryVoucher => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::CappaJuice => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::CatalystOfAeons => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::CaulfieldsWarhammer => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::CelestialOpposition => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ChainVest => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ChainlacedCrushers => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ChempunkChainsword => match attack_type {
            Melee => [chempunk_chainsword_melee_min(&ctx), zero(&ctx)],
            Ranged => [chempunk_chainsword_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::CloakOfAgility => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::CloakOfStarryNight => match attack_type {
            Melee => [cloak_of_starry_night_melee_min(&ctx), zero(&ctx)],
            Ranged => [cloak_of_starry_night_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::ClothArmor => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ControlWard => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::CosmicDrive => match attack_type {
            Melee => [cosmic_drive_melee_min(&ctx), zero(&ctx)],
            Ranged => [cosmic_drive_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::CrimsonLucidity => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::CrownOfTheShatteredQueen => match attack_type {
            Melee => [crown_of_the_shattered_queen_melee_min(&ctx), zero(&ctx)],
            Ranged => {
                [crown_of_the_shattered_queen_melee_min(&ctx), zero(&ctx)]
            }
        },

        ItemId::Cruelty => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Cryptbloom => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::CrystallineBracer => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::CrystallineOvergrowth => match attack_type {
            Melee => [crystalline_overgrowth_melee_min(&ctx), zero(&ctx)],
            Ranged => [crystalline_overgrowth_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::Cull => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Dagger => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::DarkSeal => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::DarksteelTalons => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Dawncore => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::DeadMansPlate => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::DeathsDance => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::DeathsDaughter => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Decapitator => match attack_type {
            Melee => [decapitator_melee_min(&ctx), zero(&ctx)],
            Ranged => [decapitator_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::DemonKingsCrown => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::DemonicEmbrace => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::DetonationOrb => match attack_type {
            Melee => [detonation_orb_melee_min(&ctx), zero(&ctx)],
            Ranged => [detonation_orb_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::DiademOfSongs => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::DiamondTippedSpear => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::DivineSunderer => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::DoransBlade => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::DoransBow => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::DoransHelm => match attack_type {
            Melee => [dorans_helm_melee_min(&ctx), zero(&ctx)],
            Ranged => [dorans_helm_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::DoransRing => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::DoransShield => match attack_type {
            Melee => [dorans_shield_melee_min(&ctx), zero(&ctx)],
            Ranged => [dorans_shield_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::Dragonheart => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::DreamMaker => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::DuskAndDawn => match attack_type {
            Melee => [dusk_and_dawn_melee_min(&ctx), zero(&ctx)],
            Ranged => [dusk_and_dawn_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::DuskbladeOfDraktharr => match attack_type {
            Melee => [duskblade_of_draktharr_melee_min(&ctx), zero(&ctx)],
            Ranged => [duskblade_of_draktharr_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::EchoesOfHelia => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Eclipse => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::EdgeOfNight => match attack_type {
            Melee => [edge_of_night_melee_min(&ctx), zero(&ctx)],
            Ranged => [edge_of_night_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::EleisasMiracle => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ElixirOfAvarice => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ElixirOfForce => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ElixirOfIron => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ElixirOfSkill => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ElixirOfSorcery => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ElixirOfWrath => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::EmpyreanPromise => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::EndlessHunger => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::EnhancedLuckyDice => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::EssenceReaver => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Everfrost => match attack_type {
            Melee => [everfrost_melee_min(&ctx), zero(&ctx)],
            Ranged => [everfrost_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::ExecutionersCalling => match attack_type {
            Melee => [executioners_calling_melee_min(&ctx), zero(&ctx)],
            Ranged => [executioners_calling_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::ExperimentalHexplate => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::EyeOfTheHerald => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::FaerieCharm => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::FarsightAlteration => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::FatedAshes => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::FiendhunterBolts => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::FiendishCodex => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Fimbulwinter => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::FireAtWill => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Flesheater => match attack_type {
            Melee => [flesheater_melee_min(&ctx), zero(&ctx)],
            Ranged => [flesheater_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::ForbiddenIdol => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ForceOfEntropy => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ForceOfNature => match attack_type {
            Melee => [force_of_nature_melee_min(&ctx), zero(&ctx)],
            Ranged => [force_of_nature_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::FortificationAram => match attack_type {
            Melee => [fortification_aram_melee_min(&ctx), zero(&ctx)],
            Ranged => [fortification_aram_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::FrozenHeart => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Fulmination => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Galeforce => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::GamblersBlade => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::GargoyleStoneplate => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Ghostcrawlers => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::GiantsBelt => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::GlacialBuckler => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::GlowingMote => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::GluttonousGreaves => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::GoldStatAnvilVoucher => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Goredrinker => match attack_type {
            Melee => [goredrinker_melee_min(&ctx), zero(&ctx)],
            Ranged => [goredrinker_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::GuardianAngel => match attack_type {
            Melee => [guardian_angel_melee_min(&ctx), zero(&ctx)],
            Ranged => [guardian_angel_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::GuardiansAmulet => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::GuardiansBlade => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::GuardiansDirk => match attack_type {
            Melee => [guardians_dirk_melee_min(&ctx), zero(&ctx)],
            Ranged => [guardians_dirk_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::GuardiansHammer => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::GuardiansHorn => match attack_type {
            Melee => [guardians_horn_melee_min(&ctx), zero(&ctx)],
            Ranged => [guardians_horn_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::GuardiansOrb => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::GuardiansShroud => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::GuinsoosRageblade => match attack_type {
            Melee => [guinsoos_rageblade_melee_min(&ctx), zero(&ctx)],
            Ranged => [guinsoos_rageblade_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::GunmetalGreaves => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Gusto => match attack_type {
            Melee => [gusto_melee_min(&ctx), zero(&ctx)],
            Ranged => [gusto_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::GustwalkerHatchling => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Hamstringer => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::HauntingGuise => match attack_type {
            Melee => [haunting_guise_melee_min(&ctx), zero(&ctx)],
            Ranged => [haunting_guise_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::HealthPotion => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::HearthboundAxe => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Heartsteel => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::HellfireHatchet => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::HemomancersHelm => match attack_type {
            Melee => [hemomancers_helm_melee_min(&ctx), zero(&ctx)],
            Ranged => [hemomancers_helm_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::HexboltCompanion => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Hexdrinker => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::HexopticsC44 => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::HextechAlternator => match attack_type {
            Melee => [hextech_alternator_melee_min(&ctx), zero(&ctx)],
            Ranged => [hextech_alternator_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::HextechGunblade => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::HextechRocketbelt => match attack_type {
            Melee => [hextech_rocketbelt_melee_min(&ctx), zero(&ctx)],
            Ranged => [hextech_rocketbelt_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::HollowRadiance => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::HorizonFocus => match attack_type {
            Melee => [horizon_focus_melee_min(&ctx), zero(&ctx)],
            Ranged => [horizon_focus_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::Hubris => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Hullbreaker => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::IcebornGauntlet => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ImmortalPath => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ImmortalShieldbow => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ImperialMandate => match attack_type {
            Melee => [imperial_mandate_melee_min(&ctx), zero(&ctx)],
            Ranged => [imperial_mandate_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::InfinityEdge => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::InnervatingLocket => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::IonianBootsOfLucidity => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::JakShoTheProtean => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::JarvanIs => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::JuiceOfHaste => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::JuiceOfPower => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::JuiceOfVitality => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::KaenicRookern => match attack_type {
            Melee => [kaenic_rookern_melee_min(&ctx), zero(&ctx)],
            Ranged => [kaenic_rookern_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::Kindlegem => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::KinkouJitte => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::KnightsVow => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::KrakenSlayer => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::LastWhisper => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::LegendaryAssassinItem => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::LegendaryFighterItem => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::LegendaryMageItem => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::LegendaryMarksmanItem => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::LegendarySupportItem => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::LegendaryTankItem => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::LiandrysTorment => match attack_type {
            Melee => [liandrys_torment_melee_min(&ctx), zero(&ctx)],
            Ranged => [liandrys_torment_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::LichBane => match attack_type {
            Melee => [lich_bane_melee_min(&ctx), zero(&ctx)],
            Ranged => [lich_bane_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::Lifeline => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::LightningRod => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::LocketOfTheIronSolari => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::LongSword => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::LordDominiksRegards => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::LostChapter => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::LuckyDice => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::LudensEcho => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Malignance => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Manamune => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::MawOfMalmortius => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::MejaisSoulstealer => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::MercurialScimitar => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::MercurysTreads => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::MikaelsBlessing => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::MirageBlade => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::MoonflairSpellblade => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::MoonstoneRenewer => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Morellonomicon => match attack_type {
            Melee => [morellonomicon_melee_min(&ctx), zero(&ctx)],
            Ranged => [morellonomicon_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::MortalReminder => match attack_type {
            Melee => [mortal_reminder_melee_min(&ctx), zero(&ctx)],
            Ranged => [mortal_reminder_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::MosstomperSeedling => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Multitool => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Muramana => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::NashorsTooth => match attack_type {
            Melee => [nashors_tooth_melee_min(&ctx), zero(&ctx)],
            Ranged => [nashors_tooth_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::NavoriFlickerblade => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::NeedlesslyLargeRod => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::NegatronCloak => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::NightHarvester => match attack_type {
            Melee => [night_harvester_melee_min(&ctx), zero(&ctx)],
            Ranged => [night_harvester_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::Noonquiver => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::NullMagicMantle => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::OblivionOrb => match attack_type {
            Melee => [oblivion_orb_melee_min(&ctx), zero(&ctx)],
            Ranged => [oblivion_orb_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::OhmwreckerTurretItem => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::OracleLens => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Overcharged => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::OverlordsBloodmail => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Perplexity => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Phage => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::PhantomDancer => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::PhreakishGusto => match attack_type {
            Melee => [phreakish_gusto_melee_min(&ctx), zero(&ctx)],
            Ranged => [phreakish_gusto_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::Pickaxe => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::PlatedSteelcaps => match attack_type {
            Melee => [plated_steelcaps_melee_min(&ctx), zero(&ctx)],
            Ranged => [plated_steelcaps_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::PoroSnax => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::PrismaticItem => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::PrismaticStatVoucher => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ProfaneHydra => match attack_type {
            Melee => [profane_hydra_melee_min(&ctx), zero(&ctx)],
            Ranged => [profane_hydra_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::ProtoplasmHarness => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ProwlersClaw => match attack_type {
            Melee => [prowlers_claw_melee_min(&ctx), zero(&ctx)],
            Ranged => [prowlers_claw_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::Puppeteer => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::PyromancersCloak => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::QuicksilverSash => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::RabadonsDeathcap => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::RadiantVirtue => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::RaiseMorale => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::RanduinsOmen => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::RapidFirecannon => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::RavenousHydra => match attack_type {
            Melee => [ravenous_hydra_melee_min(&ctx), zero(&ctx)],
            Ranged => [ravenous_hydra_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::RealityFracture => match attack_type {
            Melee => [reality_fracture_melee_min(&ctx), zero(&ctx)],
            Ranged => [reality_fracture_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::ReapersToll => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Rectrix => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::RecurveBow => match attack_type {
            Melee => [recurve_bow_melee_min(&ctx), zero(&ctx)],
            Ranged => [recurve_bow_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::Redemption => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::RefillablePotion => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Regicide => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ReinforcedArmorTurretItem => match attack_type {
            Melee => [reinforced_armor_turret_item_melee_min(&ctx), zero(&ctx)],
            Ranged => {
                [reinforced_armor_turret_item_melee_min(&ctx), zero(&ctx)]
            }
        },

        ItemId::RejuvenationBead => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Reverberation => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Riftmaker => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::RiteOfRuin => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::RodOfAges => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::RubyCrystal => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::RunaansHurricane => match attack_type {
            Melee => [runaans_hurricane_melee_min(&ctx), zero(&ctx)],
            Ranged => [runaans_hurricane_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::Runecarver => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::RunicCompass => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::RylaisCrystalScepter => match attack_type {
            Melee => [rylais_crystal_scepter_melee_min(&ctx), zero(&ctx)],
            Ranged => [rylais_crystal_scepter_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::SanguineGift => match attack_type {
            Melee => [sanguine_gift_melee_min(&ctx), zero(&ctx)],
            Ranged => [sanguine_gift_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::SapphireCrystal => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ScarecrowEffigy => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ScorchclawPup => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ScoutingAhead => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ScoutsSlingshot => match attack_type {
            Melee => [scouts_slingshot_melee_min(&ctx), zero(&ctx)],
            Ranged => [scouts_slingshot_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::SeekersArmguard => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::SeraphsEmbrace => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::SerpentsFang => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::SerratedDirk => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::SeryldasGrudge => match attack_type {
            Melee => [seryldas_grudge_melee_min(&ctx), zero(&ctx)],
            Ranged => [seryldas_grudge_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::Shadowflame => match attack_type {
            Melee => [shadowflame_melee_min(&ctx), zero(&ctx)],
            Ranged => [shadowflame_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::ShatteredArmguard => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Sheen => match attack_type {
            Melee => [sheen_melee_min(&ctx), zero(&ctx)],
            Ranged => [sheen_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::ShieldOfMoltenStone => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ShurelyasBattlesong => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::SlightlyMagicalBoots => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::SolsticeSleigh => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::SorcerersShoes => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::SpearOfShojin => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::SpectralCutlass => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::SpectresCowl => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::SpellslingersShoes => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::SpiritVisage => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::StaffOfFlowingWater => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::StatBonusAramMayhem => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::StatBonusArena => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::StatikkShiv => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::StealthWard => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::SteelSigil => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::SteraksGage => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Stormrazor => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Stormsurge => match attack_type {
            Melee => [stormsurge_melee_min(&ctx), zero(&ctx)],
            Ranged => [stormsurge_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::Stridebreaker => match attack_type {
            Melee => [stridebreaker_melee_min(&ctx), zero(&ctx)],
            Ranged => [stridebreaker_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::SunderedSky => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::SunfireAegis => match attack_type {
            Melee => [sunfire_aegis_melee_min(&ctx), zero(&ctx)],
            Ranged => [sunfire_aegis_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::SuperMechArmor => match attack_type {
            Melee => [super_mech_armor_melee_min(&ctx), zero(&ctx)],
            Ranged => [super_mech_armor_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::SuperMechPowerField => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Swiftmarch => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::SwordOfBlossomingDawn => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::SwordOfTheDivine => match attack_type {
            Melee => [sword_of_the_divine_melee_min(&ctx), zero(&ctx)],
            Ranged => [sword_of_the_divine_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::TalismanOfAscension => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::TearOfTheGoddess => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Terminus => match attack_type {
            Melee => [terminus_melee_min(&ctx), zero(&ctx)],
            Ranged => [terminus_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::TheBrutalizer => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::TheCollector => match attack_type {
            Melee => [the_collector_melee_min(&ctx), zero(&ctx)],
            Ranged => [the_collector_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::TheGoldenSpatula => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Thornmail => match attack_type {
            Melee => [thornmail_melee_min(&ctx), zero(&ctx)],
            Ranged => [thornmail_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::Tiamat => match attack_type {
            Melee => [tiamat_melee_min(&ctx), zero(&ctx)],
            Ranged => [tiamat_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::TitanicHydra => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::TotalBiscuitOfEverlastingWill => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::TowerPowerUp => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::TrinityForce => match attack_type {
            Melee => [trinity_force_melee_min(&ctx), zero(&ctx)],
            Ranged => [trinity_force_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::Tunneler => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::TurboChemtank => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::TurretPlating => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::TwilightsEdge => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::TwinMask => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::UmbralGlaive => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::UnendingDespair => match attack_type {
            Melee => [unending_despair_melee_min(&ctx), zero(&ctx)],
            Ranged => [unending_despair_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::VampiricScepter => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::VeigarsTalismanOfAscension => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::VerdantBarrier => match attack_type {
            Melee => [verdant_barrier_melee_min(&ctx), zero(&ctx)],
            Ranged => [verdant_barrier_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::VoidImmolation => match attack_type {
            Melee => [void_immolation_melee_min(&ctx), zero(&ctx)],
            Ranged => [void_immolation_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::VoidStaff => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::VoltaicCyclosword => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::WardensEye => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::WardensMail => match attack_type {
            Melee => [wardens_mail_melee_min(&ctx), zero(&ctx)],
            Ranged => [wardens_mail_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::WarmogsArmor => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::WhisperingCirclet => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::WingedMoonplate => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::WintersApproach => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::WitsEnd => match attack_type {
            Melee => [wits_end_melee_min(&ctx), zero(&ctx)],
            Ranged => [wits_end_melee_min(&ctx), zero(&ctx)],
        },

        ItemId::WoogletsWitchcap => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::WordlessPromise => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::WorldAtlas => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::YoumuusGhostblade => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::YourCut => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::YunTalWildarrows => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ZazZaksRealmspike => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Zeal => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ZekesConvergence => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::Zephyr => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },

        ItemId::ZhonyasHourglass => match attack_type {
            Melee => [zero(&ctx), zero(&ctx)],
            Ranged => [zero(&ctx), zero(&ctx)],
        },
    }
}
pub static ITEM_CACHE: [&Item; ItemId::VARIANTS] = [
    &ABYSSAL_MASK,
    &ACTUALIZER,
    &AETHER_WISP,
    &AMPLIFYING_TOME,
    &ANATHEMAS_CHAINS,
    &ANTI_TOWER_SOCKS,
    &ANVIL_VOUCHER,
    &ARCANE_SWEEPER_TRINKET,
    &ARCHANGELS_STAFF,
    &ARDENT_CENSER,
    &ARMORED_ADVANCE,
    &ATMAS_RECKONING,
    &AXIOM_ARC,
    &BF_SWORD,
    &BAMIS_CINDER,
    &BANDLEGLASS_MIRROR,
    &BANDLEPIPES,
    &BANSHEES_VEIL,
    &BASE_TURRET_REINFORCED_ARMOR_TURRET_ITEM,
    &BASTIONBREAKER,
    &BERSERKERS_GREAVES,
    &BLACK_CLEAVER,
    &BLACK_HOLE_GAUNTLET,
    &BLACK_SPEAR,
    &BLACKFIRE_TORCH,
    &BLADE_OF_THE_RUINED_KING,
    &BLASTING_WAND,
    &BLIGHTING_JEWEL,
    &BLOODLETTERS_CURSE,
    &BLOODSONG,
    &BLOODTHIRSTER,
    &BOOTS,
    &BOOTS_OF_SWIFTNESS,
    &BOUNTY_OF_WORLDS,
    &BRAMBLE_VEST,
    &BRAVERY_VOUCHER,
    &CAPPA_JUICE,
    &CATALYST_OF_AEONS,
    &CAULFIELDS_WARHAMMER,
    &CELESTIAL_OPPOSITION,
    &CHAIN_VEST,
    &CHAINLACED_CRUSHERS,
    &CHEMPUNK_CHAINSWORD,
    &CLOAK_OF_AGILITY,
    &CLOAK_OF_STARRY_NIGHT,
    &CLOTH_ARMOR,
    &CONTROL_WARD,
    &COSMIC_DRIVE,
    &CRIMSON_LUCIDITY,
    &CROWN_OF_THE_SHATTERED_QUEEN,
    &CRUELTY,
    &CRYPTBLOOM,
    &CRYSTALLINE_BRACER,
    &CRYSTALLINE_OVERGROWTH,
    &CULL,
    &DAGGER,
    &DARK_SEAL,
    &DARKSTEEL_TALONS,
    &DAWNCORE,
    &DEAD_MANS_PLATE,
    &DEATHS_DANCE,
    &DEATHS_DAUGHTER,
    &DECAPITATOR,
    &DEMON_KINGS_CROWN,
    &DEMONIC_EMBRACE,
    &DETONATION_ORB,
    &DIADEM_OF_SONGS,
    &DIAMOND_TIPPED_SPEAR,
    &DIVINE_SUNDERER,
    &DORANS_BLADE,
    &DORANS_BOW,
    &DORANS_HELM,
    &DORANS_RING,
    &DORANS_SHIELD,
    &DRAGONHEART,
    &DREAM_MAKER,
    &DUSK_AND_DAWN,
    &DUSKBLADE_OF_DRAKTHARR,
    &ECHOES_OF_HELIA,
    &ECLIPSE,
    &EDGE_OF_NIGHT,
    &ELEISAS_MIRACLE,
    &ELIXIR_OF_AVARICE,
    &ELIXIR_OF_FORCE,
    &ELIXIR_OF_IRON,
    &ELIXIR_OF_SKILL,
    &ELIXIR_OF_SORCERY,
    &ELIXIR_OF_WRATH,
    &EMPYREAN_PROMISE,
    &ENDLESS_HUNGER,
    &ENHANCED_LUCKY_DICE,
    &ESSENCE_REAVER,
    &EVERFROST,
    &EXECUTIONERS_CALLING,
    &EXPERIMENTAL_HEXPLATE,
    &EYE_OF_THE_HERALD,
    &FAERIE_CHARM,
    &FARSIGHT_ALTERATION,
    &FATED_ASHES,
    &FIENDHUNTER_BOLTS,
    &FIENDISH_CODEX,
    &FIMBULWINTER,
    &FIRE_AT_WILL,
    &FLESHEATER,
    &FORBIDDEN_IDOL,
    &FORCE_OF_ENTROPY,
    &FORCE_OF_NATURE,
    &FORTIFICATION_ARAM,
    &FROZEN_HEART,
    &FULMINATION,
    &GALEFORCE,
    &GAMBLERS_BLADE,
    &GARGOYLE_STONEPLATE,
    &GHOSTCRAWLERS,
    &GIANTS_BELT,
    &GLACIAL_BUCKLER,
    &GLOWING_MOTE,
    &GLUTTONOUS_GREAVES,
    &GOLD_STAT_ANVIL_VOUCHER,
    &GOREDRINKER,
    &GUARDIAN_ANGEL,
    &GUARDIANS_AMULET,
    &GUARDIANS_BLADE,
    &GUARDIANS_DIRK,
    &GUARDIANS_HAMMER,
    &GUARDIANS_HORN,
    &GUARDIANS_ORB,
    &GUARDIANS_SHROUD,
    &GUINSOOS_RAGEBLADE,
    &GUNMETAL_GREAVES,
    &GUSTO,
    &GUSTWALKER_HATCHLING,
    &HAMSTRINGER,
    &HAUNTING_GUISE,
    &HEALTH_POTION,
    &HEARTHBOUND_AXE,
    &HEARTSTEEL,
    &HELLFIRE_HATCHET,
    &HEMOMANCERS_HELM,
    &HEXBOLT_COMPANION,
    &HEXDRINKER,
    &HEXOPTICS_C_44,
    &HEXTECH_ALTERNATOR,
    &HEXTECH_GUNBLADE,
    &HEXTECH_ROCKETBELT,
    &HOLLOW_RADIANCE,
    &HORIZON_FOCUS,
    &HUBRIS,
    &HULLBREAKER,
    &ICEBORN_GAUNTLET,
    &IMMORTAL_PATH,
    &IMMORTAL_SHIELDBOW,
    &IMPERIAL_MANDATE,
    &INFINITY_EDGE,
    &INNERVATING_LOCKET,
    &IONIAN_BOOTS_OF_LUCIDITY,
    &JAK_SHO_THE_PROTEAN,
    &JARVAN_IS,
    &JUICE_OF_HASTE,
    &JUICE_OF_POWER,
    &JUICE_OF_VITALITY,
    &KAENIC_ROOKERN,
    &KINDLEGEM,
    &KINKOU_JITTE,
    &KNIGHTS_VOW,
    &KRAKEN_SLAYER,
    &LAST_WHISPER,
    &LEGENDARY_ASSASSIN_ITEM,
    &LEGENDARY_FIGHTER_ITEM,
    &LEGENDARY_MAGE_ITEM,
    &LEGENDARY_MARKSMAN_ITEM,
    &LEGENDARY_SUPPORT_ITEM,
    &LEGENDARY_TANK_ITEM,
    &LIANDRYS_TORMENT,
    &LICH_BANE,
    &LIFELINE,
    &LIGHTNING_ROD,
    &LOCKET_OF_THE_IRON_SOLARI,
    &LONG_SWORD,
    &LORD_DOMINIKS_REGARDS,
    &LOST_CHAPTER,
    &LUCKY_DICE,
    &LUDENS_ECHO,
    &MALIGNANCE,
    &MANAMUNE,
    &MAW_OF_MALMORTIUS,
    &MEJAIS_SOULSTEALER,
    &MERCURIAL_SCIMITAR,
    &MERCURYS_TREADS,
    &MIKAELS_BLESSING,
    &MIRAGE_BLADE,
    &MOONFLAIR_SPELLBLADE,
    &MOONSTONE_RENEWER,
    &MORELLONOMICON,
    &MORTAL_REMINDER,
    &MOSSTOMPER_SEEDLING,
    &MULTITOOL,
    &MURAMANA,
    &NASHORS_TOOTH,
    &NAVORI_FLICKERBLADE,
    &NEEDLESSLY_LARGE_ROD,
    &NEGATRON_CLOAK,
    &NIGHT_HARVESTER,
    &NOONQUIVER,
    &NULL_MAGIC_MANTLE,
    &OBLIVION_ORB,
    &OHMWRECKER_TURRET_ITEM,
    &ORACLE_LENS,
    &OVERCHARGED,
    &OVERLORDS_BLOODMAIL,
    &PERPLEXITY,
    &PHAGE,
    &PHANTOM_DANCER,
    &PHREAKISH_GUSTO,
    &PICKAXE,
    &PLATED_STEELCAPS,
    &PORO_SNAX,
    &PRISMATIC_ITEM,
    &PRISMATIC_STAT_VOUCHER,
    &PROFANE_HYDRA,
    &PROTOPLASM_HARNESS,
    &PROWLERS_CLAW,
    &PUPPETEER,
    &PYROMANCERS_CLOAK,
    &QUICKSILVER_SASH,
    &RABADONS_DEATHCAP,
    &RADIANT_VIRTUE,
    &RAISE_MORALE,
    &RANDUINS_OMEN,
    &RAPID_FIRECANNON,
    &RAVENOUS_HYDRA,
    &REALITY_FRACTURE,
    &REAPERS_TOLL,
    &RECTRIX,
    &RECURVE_BOW,
    &REDEMPTION,
    &REFILLABLE_POTION,
    &REGICIDE,
    &REINFORCED_ARMOR_TURRET_ITEM,
    &REJUVENATION_BEAD,
    &REVERBERATION,
    &RIFTMAKER,
    &RITE_OF_RUIN,
    &ROD_OF_AGES,
    &RUBY_CRYSTAL,
    &RUNAANS_HURRICANE,
    &RUNECARVER,
    &RUNIC_COMPASS,
    &RYLAIS_CRYSTAL_SCEPTER,
    &SANGUINE_GIFT,
    &SAPPHIRE_CRYSTAL,
    &SCARECROW_EFFIGY,
    &SCORCHCLAW_PUP,
    &SCOUTING_AHEAD,
    &SCOUTS_SLINGSHOT,
    &SEEKERS_ARMGUARD,
    &SERAPHS_EMBRACE,
    &SERPENTS_FANG,
    &SERRATED_DIRK,
    &SERYLDAS_GRUDGE,
    &SHADOWFLAME,
    &SHATTERED_ARMGUARD,
    &SHEEN,
    &SHIELD_OF_MOLTEN_STONE,
    &SHURELYAS_BATTLESONG,
    &SLIGHTLY_MAGICAL_BOOTS,
    &SOLSTICE_SLEIGH,
    &SORCERERS_SHOES,
    &SPEAR_OF_SHOJIN,
    &SPECTRAL_CUTLASS,
    &SPECTRES_COWL,
    &SPELLSLINGERS_SHOES,
    &SPIRIT_VISAGE,
    &STAFF_OF_FLOWING_WATER,
    &STAT_BONUS_ARAM_MAYHEM,
    &STAT_BONUS_ARENA,
    &STATIKK_SHIV,
    &STEALTH_WARD,
    &STEEL_SIGIL,
    &STERAKS_GAGE,
    &STORMRAZOR,
    &STORMSURGE,
    &STRIDEBREAKER,
    &SUNDERED_SKY,
    &SUNFIRE_AEGIS,
    &SUPER_MECH_ARMOR,
    &SUPER_MECH_POWER_FIELD,
    &SWIFTMARCH,
    &SWORD_OF_BLOSSOMING_DAWN,
    &SWORD_OF_THE_DIVINE,
    &TALISMAN_OF_ASCENSION,
    &TEAR_OF_THE_GODDESS,
    &TERMINUS,
    &THE_BRUTALIZER,
    &THE_COLLECTOR,
    &THE_GOLDEN_SPATULA,
    &THORNMAIL,
    &TIAMAT,
    &TITANIC_HYDRA,
    &TOTAL_BISCUIT_OF_EVERLASTING_WILL,
    &TOWER_POWER_UP,
    &TRINITY_FORCE,
    &TUNNELER,
    &TURBO_CHEMTANK,
    &TURRET_PLATING,
    &TWILIGHTS_EDGE,
    &TWIN_MASK,
    &UMBRAL_GLAIVE,
    &UNENDING_DESPAIR,
    &VAMPIRIC_SCEPTER,
    &VEIGARS_TALISMAN_OF_ASCENSION,
    &VERDANT_BARRIER,
    &VOID_IMMOLATION,
    &VOID_STAFF,
    &VOLTAIC_CYCLOSWORD,
    &WARDENS_EYE,
    &WARDENS_MAIL,
    &WARMOGS_ARMOR,
    &WHISPERING_CIRCLET,
    &WINGED_MOONPLATE,
    &WINTERS_APPROACH,
    &WITS_END,
    &WOOGLETS_WITCHCAP,
    &WORDLESS_PROMISE,
    &WORLD_ATLAS,
    &YOUMUUS_GHOSTBLADE,
    &YOUR_CUT,
    &YUN_TAL_WILDARROWS,
    &ZAZ_ZAKS_REALMSPIKE,
    &ZEAL,
    &ZEKES_CONVERGENCE,
    &ZEPHYR,
    &ZHONYAS_HOURGLASS,
];

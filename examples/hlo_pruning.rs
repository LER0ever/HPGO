use std::error::Error;
use HPGO::input::*;
// use HPGO::ir::propagate::propagate::Propagate;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use HPGO::ir::hlo_ast::Param;
use HPGO::ir::propagate::vargraph::VarGraph3D;
use HPGO::ir::*;
use std::time::{Instant, Duration};

fn get_split_vars() -> Vec<&'static str> {
    return vec![
        "%arg3451.0",
        "%arg3452.0",
        "%arg3453.0",
        "%arg3454.0",
        "%arg3455.0",
        "%arg3456.0",
        "%arg3457.0",
        "%arg3458.0",
        "%arg3459.0",
        "%arg3460.0",
        "%arg3461.0",
        "%arg3462.0",
        "%arg3463.0",
        "%arg3464.0",
        "%arg3465.0",
        "%arg3466.0",
        "%arg3467.0",
        "%arg3468.0",
        "%arg3469.0",
        "%arg3470.0",
        "%arg3471.0",
        "%arg3472.0",
        "%arg3473.0",
        "%arg3474.0",
        "%arg3475.0",
        "%arg3476.0",
        "%arg3477.0",
        "%arg3478.0",
        "%arg3479.0",
        "%arg3480.0",
        "%arg3481.0",
        "%arg3482.0",
        "%arg3483.0",
        "%arg3484.0",
        "%arg3485.0",
        "%arg3486.0",
        "%arg3487.0",
        "%arg3488.0",
        "%arg3489.0",
        "%arg3490.0",
        "%arg3491.0",
        "%arg3492.0",
        "%arg3493.0",
        "%arg3494.0",
        "%arg3495.0",
        "%arg3496.0",
        "%arg3497.0",
        "%arg3498.0",
        "%arg3499.0",
        "%arg3500.0",
        "%arg3501.0",
        "%arg3502.0",
        "%arg3503.0",
        "%arg3504.0",
        "%arg3505.0",
        "%arg3506.0",
        "%arg3507.0",
        "%arg3508.0",
        "%arg3509.0",
        "%arg3510.0",
        "%arg3511.0",
        "%arg3512.0",
        "%arg3513.0",
        "%arg3514.0",
        "%arg3515.0",
        "%arg3516.0",
        "%arg3517.0",
        "%arg3518.0",
        "%arg3519.0",
        "%arg3520.0",
        "%arg3521.0",
        "%arg3522.0",
        "%arg3523.0",
        "%arg3524.0",
        "%arg3525.0",
        "%arg3526.0",
        "%arg3527.0",
        "%arg3528.0",
        "%arg3529.0",
        "%arg3530.0",
        "%arg3531.0",
        "%arg3532.0",
        "%arg3533.0",
        "%arg3534.0",
        "%arg3535.0",
        "%arg3536.0",
        "%arg3537.0",
        "%arg3538.0",
        "%arg3539.0",
        "%arg3540.0",
        "%arg3541.0",
        "%arg3542.0",
        "%arg3543.0",
        "%arg3544.0",
        "%arg3545.0",
        "%arg3546.0",
        "%arg3547.0",
        "%arg3548.0",
        "%arg3549.0",
        "%arg3550.0",
        "%arg3551.0",
        "%arg3552.0",
        "%arg3553.0",
        "%arg3554.0",
        "%arg3555.0",
        "%arg3556.0",
        "%arg3557.0",
        "%arg3558.0",
        "%arg3559.0",
        "%arg3560.0",
        "%arg3561.0",
        "%arg3562.0",
        "%arg3563.0",
        "%arg3564.0",
        "%arg3565.0",
        "%arg3566.0",
        "%arg3567.0",
        "%arg3568.0",
        "%arg3569.0",
        "%arg3570.0",
        "%arg3571.0",
        "%arg3572.0",
        "%arg3573.0",
        "%arg3574.0",
        "%arg3575.0",
        "%arg3576.0",
        "%arg3577.0",
        "%arg3578.0",
        "%arg3579.0",
        "%arg3580.0",
        "%arg3581.0",
        "%arg3582.0",
        "%arg3583.0",
        "%arg3584.0",
        "%arg3585.0",
        "%arg3586.0",
        "%arg3587.0",
        "%arg3588.0",
        "%arg3589.0",
        "%arg3590.0",
        "%arg3591.0",
        "%arg3592.0",
        "%arg3593.0",
        "%arg3594.0",
        "%arg3595.0",
        "%arg3596.0",
        "%arg3597.0",
        "%arg3598.0",
        "%arg3599.0",
        "%arg3600.0",
        "%arg3601.0",
        "%arg3602.0",
        "%arg3603.0",
        "%arg3604.0",
        "%arg3605.0",
        "%arg3606.0",
        "%arg3607.0",
        "%arg3608.0",
        "%arg3609.0",
        "%arg3610.0",
        "%arg3611.0",
        "%arg3612.0",
        "%arg3613.0",
        "%arg3614.0",
        "%arg3615.0",
        "%arg3616.0",
        "%arg3617.0",
        "%arg3618.0",
        "%arg3619.0",
        "%arg3620.0",
        "%arg3621.0",
        "%arg3622.0",
        "%arg3623.0",
        "%arg3624.0",
        "%arg3625.0",
        "%arg3626.0",
        "%arg3627.0",
        "%arg3628.0",
        "%arg3629.0",
        "%arg3630.0",
        "%arg3631.0",
        "%arg3632.0",
        "%arg3633.0",
        "%arg3634.0",
        "%arg3635.0",
        "%arg3636.0",
        "%arg3637.0",
        "%arg3638.0",
        "%arg3639.0",
        "%arg3640.0",
        "%arg3641.0",
        "%arg3642.0",
        "%arg3643.0",
        "%arg3644.0",
        "%arg3645.0",
        "%arg3646.0",
        "%arg3647.0",
        "%arg3648.0",
        "%arg3649.0",
        "%arg3650.0",
        "%arg3651.0",
        "%arg3652.0",
        "%arg3653.0",
        "%arg3654.0",
        "%arg3655.0",
        "%arg3656.0",
        "%arg3657.0",
        "%arg3658.0",
        "%arg3659.0",
        "%arg3660.0",
        "%arg3661.0",
        "%arg3662.0",
        "%arg3663.0",
        "%arg3664.0",
        "%arg3665.0",
        "%arg3666.0",
        "%arg3667.0",
        "%arg3668.0",
        "%arg3669.0",
        "%arg3670.0",
        "%arg3671.0",
        "%arg3672.0",
        "%arg3673.0",
        "%arg3674.0",
        "%arg3675.0",
        "%arg3676.0",
        "%arg3677.0",
        "%arg3678.0",
        "%arg3679.0",
        "%arg3680.0",
        "%arg3681.0",
        "%arg3682.0",
        "%arg3683.0",
        "%arg3684.0",
        "%arg3685.0",
        "%arg3686.0",
        "%arg3687.0",
        "%arg3688.0",
        "%arg3689.0",
        "%arg3690.0",
        "%arg3691.0",
        "%arg3692.0",
        "%arg3693.0",
        "%arg3694.0",
        "%arg3695.0",
        "%arg3696.0",
        "%arg3697.0",
        "%arg3698.0",
        "%arg3699.0",
        "%arg3700.0",
        "%arg3701.0",
        "%arg3702.0",
        "%arg3703.0",
        "%arg3704.0",
        "%arg3705.0",
        "%arg3706.0",
        "%arg3707.0",
        "%arg3708.0",
        "%arg3709.0",
        "%arg3710.0",
        "%arg3711.0",
        "%arg3712.0",
        "%arg3713.0",
        "%arg3714.0",
        "%arg3715.0",
        "%arg3716.0",
        "%arg3717.0",
        "%arg3718.0",
        "%arg3719.0",
        "%arg3720.0",
        "%arg3721.0",
        "%arg3722.0",
        "%arg3723.0",
        "%arg3724.0",
        "%arg3725.0",
        "%arg3726.0",
        "%arg3727.0",
        "%arg3728.0",
        "%arg3729.0",
        "%arg3730.0",
        "%arg3731.0",
        "%arg3732.0",
        "%arg3733.0",
        "%arg3734.0",
        "%arg3735.0",
        "%arg3736.0",
        "%arg3737.0",
        "%arg3738.0",
        "%arg3739.0",
        "%arg3740.0",
        "%arg3741.0",
        "%arg3742.0",
        "%arg3743.0",
        "%arg3744.0",
        "%arg3745.0",
        "%arg3746.0",
        "%arg3747.0",
        "%arg3748.0",
        "%arg3749.0",
        "%arg3750.0",
        "%arg3751.0",
        "%arg3752.0",
        "%arg3753.0",
        "%arg3754.0",
        "%arg3755.0",
        "%arg3756.0",
        "%arg3757.0",
        "%arg3758.0",
        "%arg3759.0",
        "%arg3760.0",
        "%arg3761.0",
        "%arg3762.0",
        "%arg3763.0",
        "%arg3764.0",
        "%arg3765.0",
        "%arg3766.0",
        "%arg3767.0",
        "%arg3768.0",
        "%arg3769.0",
        "%arg3770.0",
        "%arg3771.0",
        "%arg3772.0",
        "%arg3773.0",
        "%arg3774.0",
        "%arg3775.0",
        "%arg3776.0",
        "%arg3777.0",
        "%arg3778.0",
        "%arg3779.0",
        "%arg3780.0",
        "%arg3781.0",
        "%arg3782.0",
        "%arg3783.0",
        "%arg3784.0",
        "%arg3785.0",
        "%arg3786.0",
        "%arg3787.0",
        "%arg3788.0",
        "%arg3789.0",
        "%arg3790.0",
        "%arg3791.0",
        "%arg3792.0",
        "%arg3793.0",
        "%arg3794.0",
        "%arg3795.0",
        "%arg3796.0",
        "%arg3797.0",
        "%arg3798.0",
        "%arg3799.0",
        "%arg3800.0",
        "%arg3801.0",
        "%arg3802.0",
        "%arg3803.0",
        "%arg3804.0",
        "%arg3805.0",
        "%arg3806.0",
        "%arg3807.0",
        "%arg3808.0",
        "%arg3809.0",
        "%arg3810.0",
        "%arg3811.0",
        "%arg3812.0",
        "%arg3813.0",
        "%arg3814.0",
        "%arg3815.0",
        "%arg3816.0",
        "%arg3817.0",
        "%arg3818.0",
        "%arg3819.0",
        "%arg3820.0",
        "%arg3821.0",
        "%arg3822.0",
        "%arg3823.0",
        "%arg3824.0",
        "%arg3825.0",
        "%arg3826.0",
        "%arg3827.0",
        "%arg3828.0",
        "%arg3829.0",
        "%arg3830.0",
        "%arg3831.0",
        "%arg3832.0",
        "%arg3833.0",
        "%arg3834.0",
        "%arg3835.0",
        "%arg3836.0",
        "%arg3837.0",
        "%arg3838.0",
        "%arg3839.0",
        "%arg3840.0",
        "%arg3841.0",
        "%arg3842.0",
        "%arg3843.0",
        "%arg3844.0",
        "%arg3845.0",
        "%arg3846.0",
        "%arg3847.0",
        "%arg3848.0",
        "%arg3849.0",
        "%arg3850.0",
        "%arg3851.0",
        "%arg3852.0",
        "%arg3853.0",
        "%arg3854.0",
        "%arg3855.0",
        "%arg3856.0",
        "%arg3857.0",
        "%arg3858.0",
        "%arg3859.0",
        "%arg3860.0",
        "%arg3861.0",
        "%arg3862.0",
        "%arg3863.0",
        "%arg3864.0",
        "%arg3865.0",
        "%arg3866.0",
        "%arg3867.0",
        "%arg3868.0",
        "%arg3869.0",
        "%arg3870.0",
        "%arg3871.0",
        "%arg3872.0",
        "%arg3873.0",
        "%arg3874.0",
        "%arg3875.0",
        "%arg3876.0",
        "%arg3877.0",
        "%arg3878.0",
        "%arg3879.0",
        "%arg3880.0",
        "%arg3881.0",
        "%arg3882.0",
        "%arg3883.0",
        "%arg3884.0",
        "%arg3885.0",
        "%arg3886.0",
        "%arg3887.0",
        "%arg3888.0",
        "%arg3889.0",
        "%arg3890.0",
        "%arg3891.0",
        "%arg3892.0",
        "%arg3893.0",
        "%arg3894.0",
        "%arg3895.0",
        "%arg3896.0",
        "%arg3897.0",
        "%arg3898.0",
        "%arg3899.0",
        "%arg3900.0",
        "%arg3901.0",
        "%arg3902.0",
        "%arg3903.0",
        "%arg3904.0",
        "%arg3905.0",
        "%arg3906.0",
        "%arg3907.0",
        "%arg3908.0",
        "%arg3909.0",
        "%arg3910.0",
        "%arg3911.0",
        "%arg3912.0",
        "%arg3913.0",
        "%arg3914.0",
        "%arg3915.0",
        "%arg3916.0",
        "%arg3917.0",
        "%arg3918.0",
        "%arg3919.0",
        "%arg3920.0",
        "%arg3921.0",
        "%arg3922.0",
        "%arg3923.0",
        "%arg3924.0",
        "%arg3925.0",
        "%arg3926.0",
        "%arg3927.0",
        "%arg3928.0",
        "%arg3929.0",
        "%arg3930.0",
        "%arg3931.0",
        "%arg3932.0",
        "%arg3933.0",
        "%arg3934.0",
        "%arg3935.0",
        "%arg3936.0",
        "%arg3937.0",
        "%arg3938.0",
        "%arg3939.0",
        "%arg3940.0",
        "%arg3941.0",
        "%arg3942.0",
        "%arg3943.0",
        "%arg3944.0",
        "%arg3945.0",
        "%arg3946.0",
        "%arg3947.0",
        "%arg3948.0",
        "%arg3949.0",
        "%arg3950.0",
        "%arg3951.0",
        "%arg3952.0",
        "%arg3953.0",
        "%arg3954.0",
        "%arg3955.0",
        "%arg3956.0",
        "%arg3957.0",
        "%arg3958.0",
        "%arg3959.0",
        "%arg3960.0",
        "%arg3961.0",
        "%arg3962.0",
        "%arg3963.0",
        "%arg3964.0",
        "%arg3965.0",
        "%arg3966.0",
        "%arg3967.0",
        "%arg3968.0",
        "%arg3969.0",
        "%arg3970.0",
        "%arg3971.0",
        "%arg3972.0",
        "%arg3973.0",
        "%arg3974.0",
        "%arg3975.0",
        "%arg3976.0",
        "%arg3977.0",
        "%arg3978.0",
        "%arg3979.0",
        "%arg3980.0",
        "%arg3981.0",
        "%arg3982.0",
        "%arg3983.0",
        "%arg3984.0",
        "%arg3985.0",
        "%arg3986.0",
        "%arg3987.0",
        "%arg3988.0",
        "%arg3989.0",
        "%arg3990.0",
        "%arg3991.0",
        "%arg3992.0",
        "%arg3993.0",
        "%arg3994.0",
        "%arg3995.0",
        "%arg3996.0",
        "%arg3997.0",
        "%arg3998.0",
        "%arg3999.0",
        "%arg4000.0",
        "%arg4001.0",
        "%arg4002.0",
        "%arg4003.0",
        "%arg4004.0",
        "%arg4005.0",
        "%arg4006.0",
        "%arg4007.0",
        "%arg4008.0",
        "%arg4009.0",
        "%arg4010.0",
        "%arg4011.0",
        "%arg4012.0",
        "%arg4013.0",
        "%arg4014.0",
        "%arg4015.0",
        "%arg4016.0",
        "%arg4017.0",
        "%arg4018.0",
        "%arg4019.0",
        "%arg4020.0",
        "%arg4021.0",
        "%arg4022.0",
        "%arg4023.0",
        "%arg4024.0",
        "%arg4025.0",
        "%arg4026.0",
        "%arg4027.0",
        "%arg4028.0",
        "%arg4029.0",
        "%arg4030.0",
        "%arg4031.0",
        "%arg4032.0",
        "%arg4033.0",
        "%arg4034.0",
        "%arg4035.0",
        "%arg4036.0",
        "%arg4037.0",
        "%arg4038.0",
        "%arg4039.0",
        "%arg4040.0",
        "%arg4041.0",
        "%arg4042.0",
        "%arg4043.0",
        "%arg4044.0",
        "%arg4045.0",
        "%arg4046.0",
        "%arg4047.0",
        "%arg4048.0",
        "%arg4049.0",
        "%arg4050.0",
        "%arg4051.0",
        "%arg4052.0",
        "%arg4053.0",
        "%arg4054.0",
        "%arg4055.0",
        "%arg4056.0",
        "%arg4057.0",
        "%arg4058.0",
        "%arg4059.0",
        "%arg4060.0",
        "%arg4061.0",
        "%arg4062.0",
        "%arg4063.0",
        "%arg4064.0",
        "%arg4065.0",
        "%arg4066.0",
        "%arg4067.0",
        "%arg4068.0",
        "%arg4069.0",
        "%arg4070.0",
        "%arg4071.0",
        "%arg4072.0",
        "%arg4073.0",
        "%arg4074.0",
        "%arg4075.0",
        "%arg4076.0",
        "%arg4077.0",
        "%arg4078.0",
        "%arg4079.0",
        "%arg4080.0",
        "%arg4081.0",
        "%arg4082.0",
        "%arg4083.0",
        "%arg4084.0",
        "%arg4085.0",
        "%arg4086.0",
        "%arg4087.0",
        "%arg4088.0",
        "%arg4089.0",
        "%arg4090.0",
        "%arg4091.0",
        "%arg4092.0",
        "%arg4093.0",
        "%arg4094.0",
        "%arg4095.0",
        "%arg4096.0",
        "%arg4097.0",
        "%arg4098.0",
        "%arg4099.0",
        "%arg4100.0",
        "%arg4101.0",
        "%arg4102.0",
        "%arg4103.0",
        "%arg4104.0",
        "%arg4105.0",
        "%arg4106.0",
        "%arg4107.0",
        "%arg4108.0",
        "%arg4109.0",
        "%arg4110.0",
        "%arg4111.0",
        "%arg4112.0",
        "%arg4113.0",
        "%arg4114.0",
        "%arg4115.0",
        "%arg4116.0",
        "%arg4117.0",
        "%arg4118.0",
        "%arg4119.0",
        "%arg4120.0",
        "%arg4121.0",
        "%arg4122.0",
        "%arg4123.0",
        "%arg4124.0",
        "%arg4125.0",
        "%arg4126.0",
        "%arg4127.0",
        "%arg4128.0",
        "%arg4129.0",
        "%arg4130.0",
        "%arg4131.0",
        "%arg4132.0",
        "%arg4133.0",
        "%arg4134.0",
        "%arg4135.0",
        "%arg4136.0",
        "%arg4137.0",
        "%arg4138.0",
        "%arg4139.0",
        "%arg4140.0",
        "%arg4141.0",
        "%arg4142.0",
        "%arg4143.0",
        "%arg4144.0",
        "%arg4145.0",
        "%arg4146.0",
        "%arg4147.0",
        "%arg4148.0",
        "%arg4149.0",
        "%arg4150.0",
        "%arg4151.0",
        "%arg4152.0",
        "%arg4153.0",
        "%arg4154.0",
        "%arg4155.0",
        "%arg4156.0",
        "%arg4157.0",
        "%arg4158.0",
        "%arg4159.0",
        "%arg4160.0",
        "%arg4161.0",
        "%arg4162.0",
        "%arg4163.0",
        "%arg4164.0",
        "%arg4165.0",
        "%arg4166.0",
        "%arg4167.0",
        "%arg4168.0",
        "%arg4169.0",
        "%arg4170.0",
        "%arg4171.0",
        "%arg4172.0",
        "%arg4173.0",
        "%arg4174.0",
        "%arg4175.0",
        "%arg4176.0",
        "%arg4177.0",
        "%arg4178.0",
    ];
}

fn get_qkv_list() -> Vec<&'static str> {
    return vec![
        "%arg3453.0",
        "%arg3455.0",
        "%arg3456.0",
        "%arg3459.0",
        "%arg3465.0",
        "%arg3466.0",
        "%arg3467.0",
        "%arg3477.0",
        "%arg3478.0",
        "%arg3479.0",
        "%arg3489.0",
        "%arg3490.0",
        "%arg3491.0",
        "%arg3501.0",
        "%arg3502.0",
        "%arg3503.0",
        "%arg3513.0",
        "%arg3514.0",
        "%arg3515.0",
        "%arg3525.0",
        "%arg3526.0",
        "%arg3527.0",
        "%arg3537.0",
        "%arg3538.0",
        "%arg3539.0",
        "%arg3549.0",
        "%arg3550.0",
        "%arg3551.0",
        "%arg3561.0",
        "%arg3562.0",
        "%arg3563.0",
        "%arg3573.0",
        "%arg3574.0",
        "%arg3575.0",
        "%arg3585.0",
        "%arg3586.0",
        "%arg3587.0",
        "%arg3597.0",
        "%arg3598.0",
        "%arg3599.0",
        "%arg3609.0",
        "%arg3610.0",
        "%arg3611.0",
        "%arg3621.0",
        "%arg3622.0",
        "%arg3623.0",
        "%arg3633.0",
        "%arg3634.0",
        "%arg3635.0",
        "%arg3645.0",
        "%arg3646.0",
        "%arg3647.0",
        "%arg3657.0",
        "%arg3658.0",
        "%arg3659.0",
        "%arg3669.0",
        "%arg3670.0",
        "%arg3671.0",
        "%arg3681.0",
        "%arg3682.0",
        "%arg3683.0",
        "%arg3693.0",
        "%arg3694.0",
        "%arg3695.0",
        "%arg3705.0",
        "%arg3706.0",
        "%arg3707.0",
        "%arg3717.0",
        "%arg3718.0",
        "%arg3719.0",
        "%arg3729.0",
        "%arg3730.0",
        "%arg3731.0",
        "%arg3741.0",
        "%arg3742.0",
        "%arg3743.0",
        "%arg3753.0",
        "%arg3754.0",
        "%arg3763.0",
        "%arg3765.0",
        "%arg3766.0",
        "%arg3769.0",
        "%arg3771.0",
        "%arg3772.0",
        "%arg3781.0",
        "%arg3783.0",
        "%arg3784.0",
        "%arg3787.0",
        "%arg3789.0",
        "%arg3790.0",
        "%arg3799.0",
        "%arg3801.0",
        "%arg3802.0",
        "%arg3805.0",
        "%arg3807.0",
        "%arg3808.0",
        "%arg3817.0",
        "%arg3819.0",
        "%arg3820.0",
        "%arg3823.0",
        "%arg3825.0",
        "%arg3826.0",
        "%arg3835.0",
        "%arg3837.0",
        "%arg3838.0",
        "%arg3841.0",
        "%arg3843.0",
        "%arg3844.0",
        "%arg3853.0",
        "%arg3855.0",
        "%arg3856.0",
        "%arg3859.0",
        "%arg3861.0",
        "%arg3862.0",
        "%arg3871.0",
        "%arg3873.0",
        "%arg3874.0",
        "%arg3877.0",
        "%arg3879.0",
        "%arg3880.0",
        "%arg3889.0",
        "%arg3891.0",
        "%arg3892.0",
        "%arg3895.0",
        "%arg3897.0",
        "%arg3898.0",
        "%arg3907.0",
        "%arg3909.0",
        "%arg3910.0",
        "%arg3913.0",
        "%arg3915.0",
        "%arg3916.0",
        "%arg3925.0",
        "%arg3927.0",
        "%arg3928.0",
        "%arg3931.0",
        "%arg3933.0",
        "%arg3934.0",
        "%arg3943.0",
        "%arg3945.0",
        "%arg3946.0",
        "%arg3949.0",
        "%arg3951.0",
        "%arg3952.0",
        "%arg3961.0",
        "%arg3963.0",
        "%arg3964.0",
        "%arg3967.0",
        "%arg3969.0",
        "%arg3970.0",
        "%arg3979.0",
        "%arg3981.0",
        "%arg3982.0",
        "%arg3985.0",
        "%arg3987.0",
        "%arg3988.0",
        "%arg3997.0",
        "%arg3999.0",
        "%arg4000.0",
        "%arg4003.0",
        "%arg4005.0",
        "%arg4006.0",
        "%arg4015.0",
        "%arg4017.0",
        "%arg4018.0",
        "%arg4021.0",
        "%arg4023.0",
        "%arg4024.0",
        "%arg4033.0",
        "%arg4035.0",
        "%arg4036.0",
        "%arg4039.0",
        "%arg4041.0",
        "%arg4042.0",
        "%arg4051.0",
        "%arg4053.0",
        "%arg4054.0",
        "%arg4057.0",
        "%arg4059.0",
        "%arg4060.0",
        "%arg4069.0",
        "%arg4071.0",
        "%arg4072.0",
        "%arg4075.0",
        "%arg4077.0",
        "%arg4078.0",
        "%arg4087.0",
        "%arg4089.0",
        "%arg4090.0",
        "%arg4093.0",
        "%arg4095.0",
        "%arg4096.0",
        "%arg4105.0",
        "%arg4107.0",
        "%arg4108.0",
        "%arg4111.0",
        "%arg4113.0",
        "%arg4114.0",
        "%arg4123.0",
        "%arg4125.0",
        "%arg4126.0",
        "%arg4129.0",
        "%arg4131.0",
        "%arg4132.0",
        "%arg4141.0",
        "%arg4143.0",
        "%arg4144.0",
        "%arg4147.0",
        "%arg4149.0",
        "%arg4150.0",
        "%arg4159.0",
        "%arg4161.0",
        "%arg4162.0",
        "%arg4165.0",
        "%arg4167.0",
        "%arg4168.0",
    ];
}

fn main() -> Result<(), Box<dyn Error>> {
    let hi: hlo_string::HLOStructuredJsonImporter = HLOModelImporter::new();
    let ast = hi.ImportFrom("./tests/test_data/hlo/hlo.json")?;
    let mut d = derive::Derivation::new_with_ast(&ast);
    d.cache_all_derive(&ast)?;
    let mut g = VarGraph3D::new(&d);
    // g.build_from_function("%cluster_0__XlaCompiledKernel_true__XlaNumConstantArgs_8315__XlaNumResourceArgs_2186_.94957.ComputeTask")?;
    // g.build_from_function("%fused_computation.2271.clone")?;

    g.build_from_hlo()?;
    g.update_graph_for_fusion()?;

    let split_vars: HashSet<&'static str> = get_split_vars().iter().cloned().collect();
    let mut qkv_constraits: BTreeMap<&'static str, BTreeSet<i8>> = BTreeMap::new();
    for x in get_qkv_list() {
        qkv_constraits.insert(x, vec![1i8].iter().cloned().collect());
    }

    // let fn_name = "%fused_computation.2271.clone";
    let fn_name = "%cluster_0__XlaCompiledKernel_true__XlaNumConstantArgs_8315__XlaNumResourceArgs_2186_.94957.ComputeTask";
    let f = g.ast.functions.iter().find(|f| f.name == fn_name).unwrap();
    let mut target_params: Vec<Param> = vec![];
    f.params.iter().for_each(|p| {
        if split_vars.contains(p.name.as_str())
        /*&& !qkv_constraits.contains_key(p.name.as_str())*/
        {
            target_params.push(p.clone());
        }
    });
    println!(
        "got {} target params out of {} all",
        target_params.len(),
        split_vars.len()
    );
    let now = Instant::now();
    let result = g.propagate_remt(0, &BTreeMap::new(), &target_params, None)?;
    println!(
        "[propagation]\t Propagate REMT on AST Root... {}s",
        now.elapsed().as_secs()
    );
    println!("main returns {} results", result.len());
    for (i, r) in result.iter().enumerate() {
        println!("{} :: {:?}", i, r);
    }

    // print!("{}", g.export_to_dot()?);
    // print!("Matrix: {:#?}", g.g.adjacency_matrix());
    Ok(())
    // as long as unwrap succeeds
    // println!("{:#?}", result);
}

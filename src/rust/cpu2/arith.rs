use cpu2::cpu::*;
use cpu2::global_pointers::*;
use cpu2::memory::{read8, write8};
use cpu2::misc_instr::{getaf, getcf};

pub fn int_log2(x: i32) -> i32 { 31 - x.leading_zeros() as i32 }

#[no_mangle]
pub unsafe fn add(dest_operand: i32, source_operand: i32, op_size: i32) -> i32 {
    *last_op1 = dest_operand;
    *last_op2 = source_operand;
    let res: i32 = dest_operand + source_operand;
    *last_result = res;
    *last_add_result = *last_result;
    *last_op_size = op_size;
    *flags_changed = FLAGS_ALL;
    return res;
}
#[no_mangle]
pub unsafe fn adc(dest_operand: i32, source_operand: i32, op_size: i32) -> i32 {
    let cf: i32 = getcf() as i32;
    *last_op1 = dest_operand;
    *last_op2 = source_operand;
    let res: i32 = dest_operand + source_operand + cf;
    *last_result = res;
    *last_add_result = *last_result;
    *last_op_size = op_size;
    *flags_changed = FLAGS_ALL;
    return res;
}
#[no_mangle]
pub unsafe fn sub(dest_operand: i32, source_operand: i32, op_size: i32) -> i32 {
    *last_add_result = dest_operand;
    *last_op2 = source_operand;
    let res: i32 = dest_operand - source_operand;
    *last_result = res;
    *last_op1 = *last_result;
    *last_op_size = op_size;
    *flags_changed = FLAGS_ALL;
    return res;
}
#[no_mangle]
pub unsafe fn sbb(dest_operand: i32, source_operand: i32, op_size: i32) -> i32 {
    let cf: i32 = getcf() as i32;
    *last_add_result = dest_operand;
    *last_op2 = source_operand;
    let res: i32 = dest_operand - source_operand - cf;
    *last_result = res;
    *last_op1 = *last_result;
    *last_op_size = op_size;
    *flags_changed = FLAGS_ALL;
    return res;
}
#[no_mangle]
pub unsafe fn add8(x: i32, y: i32) -> i32 { return add(x, y, OPSIZE_8); }
#[no_mangle]
pub unsafe fn add16(x: i32, y: i32) -> i32 { return add(x, y, OPSIZE_16); }
#[no_mangle]
pub unsafe fn add32(x: i32, y: i32) -> i32 { return add(x, y, OPSIZE_32); }
#[no_mangle]
pub unsafe fn sub8(x: i32, y: i32) -> i32 { return sub(x, y, OPSIZE_8); }
#[no_mangle]
pub unsafe fn sub16(x: i32, y: i32) -> i32 { return sub(x, y, OPSIZE_16); }
#[no_mangle]
pub unsafe fn sub32(x: i32, y: i32) -> i32 { return sub(x, y, OPSIZE_32); }
#[no_mangle]
pub unsafe fn adc8(x: i32, y: i32) -> i32 { return adc(x, y, OPSIZE_8); }
#[no_mangle]
pub unsafe fn adc16(x: i32, y: i32) -> i32 { return adc(x, y, OPSIZE_16); }
#[no_mangle]
pub unsafe fn adc32(x: i32, y: i32) -> i32 { return adc(x, y, OPSIZE_32); }
#[no_mangle]
pub unsafe fn sbb8(x: i32, y: i32) -> i32 { return sbb(x, y, OPSIZE_8); }
#[no_mangle]
pub unsafe fn sbb16(x: i32, y: i32) -> i32 { return sbb(x, y, OPSIZE_16); }
#[no_mangle]
pub unsafe fn sbb32(x: i32, y: i32) -> i32 { return sbb(x, y, OPSIZE_32); }
#[no_mangle]
pub unsafe fn cmp8(x: i32, y: i32) { sub(x, y, OPSIZE_8); }
#[no_mangle]
pub unsafe fn cmp16(x: i32, y: i32) { sub(x, y, OPSIZE_16); }
#[no_mangle]
pub unsafe fn cmp32(x: i32, y: i32) { sub(x, y, OPSIZE_32); }
#[no_mangle]
pub unsafe fn inc(dest_operand: i32, op_size: i32) -> i32 {
    *flags = *flags & !1 | getcf() as i32;
    *last_op1 = dest_operand;
    *last_op2 = 1;
    let res: i32 = dest_operand + 1;
    *last_result = res;
    *last_add_result = *last_result;
    *last_op_size = op_size;
    *flags_changed = FLAGS_ALL & !1;
    return res;
}
#[no_mangle]
pub unsafe fn dec(dest_operand: i32, op_size: i32) -> i32 {
    *flags = *flags & !1 | getcf() as i32;
    *last_add_result = dest_operand;
    *last_op2 = 1;
    let res: i32 = dest_operand - 1;
    *last_result = res;
    *last_op1 = *last_result;
    *last_op_size = op_size;
    *flags_changed = FLAGS_ALL & !1;
    return res;
}
#[no_mangle]
pub unsafe fn inc8(x: i32) -> i32 { return inc(x, OPSIZE_8); }
#[no_mangle]
pub unsafe fn inc16(x: i32) -> i32 { return inc(x, OPSIZE_16); }
#[no_mangle]
pub unsafe fn inc32(x: i32) -> i32 { return inc(x, OPSIZE_32); }
#[no_mangle]
pub unsafe fn dec8(x: i32) -> i32 { return dec(x, OPSIZE_8); }
#[no_mangle]
pub unsafe fn dec16(x: i32) -> i32 { return dec(x, OPSIZE_16); }
#[no_mangle]
pub unsafe fn dec32(x: i32) -> i32 { return dec(x, OPSIZE_32); }
#[no_mangle]
pub unsafe fn neg(dest_operand: i32, op_size: i32) -> i32 {
    let res: i32 = -dest_operand;
    *last_result = res;
    *last_op1 = *last_result;
    *flags_changed = FLAGS_ALL;
    *last_add_result = 0;
    *last_op2 = dest_operand;
    *last_op_size = op_size;
    return res;
}
#[no_mangle]
pub unsafe fn neg8(x: i32) -> i32 { return neg(x, OPSIZE_8); }
#[no_mangle]
pub unsafe fn neg16(x: i32) -> i32 { return neg(x, OPSIZE_16); }
#[no_mangle]
pub unsafe fn neg32(x: i32) -> i32 { return neg(x, OPSIZE_32); }
#[no_mangle]
pub unsafe fn mul8(source_operand: i32) {
    let result: i32 = source_operand * *reg8.offset(AL as isize) as i32;
    *reg16.offset(AX as isize) = result as u16;
    *last_result = result & 255;
    *last_op_size = OPSIZE_8;
    if result < 256 {
        *flags = *flags & !1 & !FLAG_OVERFLOW
    }
    else {
        *flags = *flags | 1 | FLAG_OVERFLOW
    }
    *flags_changed = FLAGS_ALL & !1 & !FLAG_OVERFLOW;
}
#[no_mangle]
pub unsafe fn imul8(source_operand: i32) {
    let result: i32 = source_operand * *reg8s.offset(AL as isize) as i32;
    *reg16.offset(AX as isize) = result as u16;
    *last_result = result & 255;
    *last_op_size = OPSIZE_8;
    if result > 127 || result < -128 {
        *flags = *flags | 1 | FLAG_OVERFLOW
    }
    else {
        *flags = *flags & !1 & !FLAG_OVERFLOW
    }
    *flags_changed = FLAGS_ALL & !1 & !FLAG_OVERFLOW;
}
#[no_mangle]
pub unsafe fn mul16(source_operand: u32) {
    let result: u32 = source_operand.wrapping_mul(*reg16.offset(AX as isize) as u32);
    let high_result: u32 = result >> 16;
    *reg16.offset(AX as isize) = result as u16;
    *reg16.offset(DX as isize) = high_result as u16;
    *last_result = (result & 0xFFFF) as i32;
    *last_op_size = OPSIZE_16;
    if high_result == 0 {
        *flags &= !1 & !FLAG_OVERFLOW
    }
    else {
        *flags |= *flags | 1 | FLAG_OVERFLOW
    }
    *flags_changed = FLAGS_ALL & !1 & !FLAG_OVERFLOW;
}
#[no_mangle]
pub unsafe fn imul16(source_operand: i32) {
    let result: i32 = source_operand * *reg16s.offset(AX as isize) as i32;
    *reg16.offset(AX as isize) = result as u16;
    *reg16.offset(DX as isize) = (result >> 16) as u16;
    *last_result = result & 0xFFFF;
    *last_op_size = OPSIZE_16;
    if result > 32767 || result < -32768 {
        *flags |= 1 | FLAG_OVERFLOW
    }
    else {
        *flags &= !1 & !FLAG_OVERFLOW
    }
    *flags_changed = FLAGS_ALL & !1 & !FLAG_OVERFLOW;
}
#[no_mangle]
pub unsafe fn imul_reg16(mut operand1: i32, mut operand2: i32) -> i32 {
    operand1 = operand1 << 16 >> 16;
    operand2 = operand2 << 16 >> 16;
    let result: i32 = operand1 * operand2;
    *last_result = result & 0xFFFF;
    *last_op_size = OPSIZE_16;
    if result > 32767 || result < -32768 {
        *flags |= 1 | FLAG_OVERFLOW
    }
    else {
        *flags &= !1 & !FLAG_OVERFLOW
    }
    *flags_changed = FLAGS_ALL & !1 & !FLAG_OVERFLOW;
    return result;
}
#[no_mangle]
pub unsafe fn mul32(source_operand: i32) {
    let dest_operand: i32 = *reg32s.offset(EAX as isize);
    let result: u64 = (dest_operand as u32 as u64).wrapping_mul(source_operand as u32 as u64);
    let result_low: i32 = result as i32;
    let result_high: i32 = (result >> 32) as i32;
    *reg32s.offset(EAX as isize) = result_low;
    *reg32s.offset(EDX as isize) = result_high;
    *last_result = result_low;
    *last_op_size = OPSIZE_32;
    if result_high == 0 {
        *flags &= !1 & !FLAG_OVERFLOW
    }
    else {
        *flags |= 1 | FLAG_OVERFLOW
    }
    *flags_changed = FLAGS_ALL & !1 & !FLAG_OVERFLOW;
}
#[no_mangle]
pub unsafe fn imul32(source_operand: i32) {
    let dest_operand: i32 = *reg32s.offset(EAX as isize);
    let result: i64 = dest_operand as i64 * source_operand as i64;
    let result_low: i32 = result as i32;
    let result_high: i32 = (result >> 32) as i32;
    *reg32s.offset(EAX as isize) = result_low;
    *reg32s.offset(EDX as isize) = result_high;
    *last_result = result_low;
    *last_op_size = OPSIZE_32;
    if result_high == result_low >> 31 {
        *flags &= !1 & !FLAG_OVERFLOW
    }
    else {
        *flags |= 1 | FLAG_OVERFLOW
    }
    *flags_changed = FLAGS_ALL & !1 & !FLAG_OVERFLOW;
}
#[no_mangle]
pub unsafe fn imul_reg32(operand1: i32, operand2: i32) -> i32 {
    let result: i64 = operand1 as i64 * operand2 as i64;
    let result_low: i32 = result as i32;
    let result_high: i32 = (result >> 32) as i32;
    *last_result = result_low;
    *last_op_size = OPSIZE_32;
    if result_high == result_low >> 31 {
        *flags &= !1 & !FLAG_OVERFLOW
    }
    else {
        *flags |= 1 | FLAG_OVERFLOW
    }
    *flags_changed = FLAGS_ALL & !1 & !FLAG_OVERFLOW;
    return result_low;
}
#[no_mangle]
pub unsafe fn xadd8(source_operand: i32, reg: i32) -> i32 {
    let tmp: i32 = *reg8.offset(reg as isize) as i32;
    *reg8.offset(reg as isize) = source_operand as u8;
    return add(source_operand, tmp, OPSIZE_8);
}
#[no_mangle]
pub unsafe fn xadd16(source_operand: i32, reg: i32) -> i32 {
    let tmp: i32 = *reg16.offset(reg as isize) as i32;
    *reg16.offset(reg as isize) = source_operand as u16;
    return add(source_operand, tmp, OPSIZE_16);
}
#[no_mangle]
pub unsafe fn xadd32(source_operand: i32, reg: i32) -> i32 {
    let tmp: i32 = *reg32s.offset(reg as isize);
    *reg32s.offset(reg as isize) = source_operand;
    return add(source_operand, tmp, OPSIZE_32);
}
#[no_mangle]
pub unsafe fn bcd_daa() {
    let old_al: i32 = *reg8.offset(AL as isize) as i32;
    let old_cf: i32 = getcf() as i32;
    let old_af: i32 = getaf() as i32;
    *flags &= !1 & !FLAG_ADJUST;
    if old_al & 15 > 9 || 0 != old_af {
        *reg8.offset(AL as isize) += 6;
        *flags |= FLAG_ADJUST
    }
    if old_al > 153 || 0 != old_cf {
        *reg8.offset(AL as isize) += 96;
        *flags |= 1
    }
    *last_result = *reg8.offset(AL as isize) as i32;
    *last_op_size = OPSIZE_8;
    *last_op2 = 0;
    *last_op1 = *last_op2;
    *flags_changed = FLAGS_ALL & !1 & !FLAG_ADJUST & !FLAG_OVERFLOW;
}
#[no_mangle]
pub unsafe fn bcd_das() {
    let old_al: i32 = *reg8.offset(AL as isize) as i32;
    let old_cf: i32 = getcf() as i32;
    *flags &= !1;
    if old_al & 15 > 9 || 0 != getaf() as i32 {
        *reg8.offset(AL as isize) -= 6;
        *flags |= FLAG_ADJUST;
        *flags = *flags & !1 | old_cf | (old_al < 6) as i32
    }
    else {
        *flags &= !FLAG_ADJUST
    }
    if old_al > 153 || 0 != old_cf {
        *reg8.offset(AL as isize) -= 96;
        *flags |= 1
    }
    *last_result = *reg8.offset(AL as isize) as i32;
    *last_op_size = OPSIZE_8;
    *last_op2 = 0;
    *last_op1 = *last_op2;
    *flags_changed = FLAGS_ALL & !1 & !FLAG_ADJUST & !FLAG_OVERFLOW;
}
#[no_mangle]
pub unsafe fn bcd_aad(imm8: i32) {
    let result: i32 = *reg8.offset(AL as isize) as i32 + *reg8.offset(AH as isize) as i32 * imm8;
    *last_result = result & 255;
    *reg16.offset(AX as isize) = *last_result as u16;
    *last_op_size = OPSIZE_8;
    *flags_changed = FLAGS_ALL & !1 & !FLAG_ADJUST & !FLAG_OVERFLOW;
    *flags &= !1 & !FLAG_ADJUST & !FLAG_OVERFLOW;
    if result > 0xFFFF {
        *flags |= 1
    };
}
#[no_mangle]
pub unsafe fn bcd_aam(imm8: i32) {
    // ascii adjust after multiplication
    if imm8 == 0 {
        trigger_de();
    }
    else {
        let temp: u8 = *reg8.offset(AL as isize);
        *reg8.offset(AH as isize) = (temp as i32 / imm8) as u8;
        *reg8.offset(AL as isize) = (temp as i32 % imm8) as u8;
        *last_result = *reg8.offset(AL as isize) as i32;
        *flags_changed = FLAGS_ALL & !1 & !FLAG_ADJUST & !FLAG_OVERFLOW;
        *flags &= !1 & !FLAG_ADJUST & !FLAG_OVERFLOW
    };
}
#[no_mangle]
pub unsafe fn bcd_aaa() {
    if *reg8.offset(AL as isize) as i32 & 15 > 9 || 0 != getaf() as i32 {
        *reg16.offset(AX as isize) += 6;
        *reg8.offset(AH as isize) += 1;
        *flags |= FLAG_ADJUST | 1
    }
    else {
        *flags &= !FLAG_ADJUST & !1
    }
    *reg8.offset(AL as isize) &= 15;
    *flags_changed &= !FLAG_ADJUST & !1;
}
#[no_mangle]
pub unsafe fn bcd_aas() {
    if *reg8.offset(AL as isize) as i32 & 15 > 9 || 0 != getaf() as i32 {
        *reg16.offset(AX as isize) -= 6;
        *reg8.offset(AH as isize) -= 1;
        *flags |= FLAG_ADJUST | 1
    }
    else {
        *flags &= !FLAG_ADJUST & !1
    }
    *reg8.offset(AL as isize) &= 15;
    *flags_changed &= !FLAG_ADJUST & !1;
}
#[no_mangle]
pub unsafe fn and(dest_operand: i32, source_operand: i32, op_size: i32) -> i32 {
    *last_result = dest_operand & source_operand;
    *last_op_size = op_size;
    *flags &= !1 & !FLAG_OVERFLOW & !FLAG_ADJUST;
    *flags_changed = FLAGS_ALL & !1 & !FLAG_OVERFLOW & !FLAG_ADJUST;
    return *last_result;
}
#[no_mangle]
pub unsafe fn or(dest_operand: i32, source_operand: i32, op_size: i32) -> i32 {
    *last_result = dest_operand | source_operand;
    *last_op_size = op_size;
    *flags &= !1 & !FLAG_OVERFLOW & !FLAG_ADJUST;
    *flags_changed = FLAGS_ALL & !1 & !FLAG_OVERFLOW & !FLAG_ADJUST;
    return *last_result;
}
#[no_mangle]
pub unsafe fn xor(dest_operand: i32, source_operand: i32, op_size: i32) -> i32 {
    *last_result = dest_operand ^ source_operand;
    *last_op_size = op_size;
    *flags &= !1 & !FLAG_OVERFLOW & !FLAG_ADJUST;
    *flags_changed = FLAGS_ALL & !1 & !FLAG_OVERFLOW & !FLAG_ADJUST;
    return *last_result;
}
#[no_mangle]
pub unsafe fn and8(x: i32, y: i32) -> i32 { return and(x, y, OPSIZE_8); }
#[no_mangle]
pub unsafe fn and16(x: i32, y: i32) -> i32 { return and(x, y, OPSIZE_16); }
#[no_mangle]
pub unsafe fn and32(x: i32, y: i32) -> i32 { return and(x, y, OPSIZE_32); }
#[no_mangle]
pub unsafe fn test8(x: i32, y: i32) { and(x, y, OPSIZE_8); }
#[no_mangle]
pub unsafe fn test16(x: i32, y: i32) { and(x, y, OPSIZE_16); }
#[no_mangle]
pub unsafe fn test32(x: i32, y: i32) { and(x, y, OPSIZE_32); }
#[no_mangle]
pub unsafe fn or8(x: i32, y: i32) -> i32 { return or(x, y, OPSIZE_8); }
#[no_mangle]
pub unsafe fn or16(x: i32, y: i32) -> i32 { return or(x, y, OPSIZE_16); }
#[no_mangle]
pub unsafe fn or32(x: i32, y: i32) -> i32 { return or(x, y, OPSIZE_32); }
#[no_mangle]
pub unsafe fn xor8(x: i32, y: i32) -> i32 { return xor(x, y, OPSIZE_8); }
#[no_mangle]
pub unsafe fn xor16(x: i32, y: i32) -> i32 { return xor(x, y, OPSIZE_16); }
#[no_mangle]
pub unsafe fn xor32(x: i32, y: i32) -> i32 { return xor(x, y, OPSIZE_32); }

#[no_mangle]
pub unsafe fn rol8(dest_operand: i32, mut count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    if 0 == count {
        return dest_operand;
    }
    else {
        count &= 7;
        let result: i32 = dest_operand << count | dest_operand >> 8 - count;
        *flags_changed &= !1 & !FLAG_OVERFLOW;
        *flags = *flags & !1 & !FLAG_OVERFLOW
            | result & 1
            | (result << 11 ^ result << 4) & FLAG_OVERFLOW;
        return result;
    };
}
#[no_mangle]
pub unsafe fn rol16(dest_operand: i32, mut count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    if 0 == count {
        return dest_operand;
    }
    else {
        count &= 15;
        let result: i32 = dest_operand << count | dest_operand >> 16 - count;
        *flags_changed &= !1 & !FLAG_OVERFLOW;
        *flags = *flags & !1 & !FLAG_OVERFLOW
            | result & 1
            | (result << 11 ^ result >> 4) & FLAG_OVERFLOW;
        return result;
    };
}
#[no_mangle]
pub unsafe fn rol32(dest_operand: i32, count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    if 0 == count {
        return dest_operand;
    }
    else {
        let result: i32 =
            ((dest_operand << count) as u32 | dest_operand as u32 >> 32 - count) as i32;
        *flags_changed &= !1 & !FLAG_OVERFLOW;
        *flags = *flags & !1 & !FLAG_OVERFLOW
            | result & 1
            | (result << 11 ^ result >> 20) & FLAG_OVERFLOW;
        return result;
    };
}
#[no_mangle]
pub unsafe fn rcl8(dest_operand: i32, mut count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    count %= 9;
    if 0 == count {
        return dest_operand;
    }
    else {
        let result: i32 =
            dest_operand << count | (getcf() as i32) << count - 1 | dest_operand >> 9 - count;
        *flags_changed &= !1 & !FLAG_OVERFLOW;
        *flags = *flags & !1 & !FLAG_OVERFLOW
            | result >> 8 & 1
            | (result << 3 ^ result << 4) & FLAG_OVERFLOW;
        return result;
    };
}
#[no_mangle]
pub unsafe fn rcl16(dest_operand: i32, mut count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    count %= 17;
    if 0 == count {
        return dest_operand;
    }
    else {
        let result: i32 =
            dest_operand << count | (getcf() as i32) << count - 1 | dest_operand >> 17 - count;
        *flags_changed &= !1 & !FLAG_OVERFLOW;
        *flags = *flags & !1 & !FLAG_OVERFLOW
            | result >> 16 & 1
            | (result >> 5 ^ result >> 4) & FLAG_OVERFLOW;
        return result;
    };
}
#[no_mangle]
pub unsafe fn rcl32(dest_operand: i32, count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    if 0 == count {
        return dest_operand;
    }
    else {
        let mut result: i32 = dest_operand << count | (getcf() as i32) << count - 1;
        if count > 1 {
            result = (result as u32 | dest_operand as u32 >> 33 - count) as i32
        }
        *flags_changed &= !1 & !FLAG_OVERFLOW;
        *flags =
            ((*flags & !1 & !FLAG_OVERFLOW) as u32 | dest_operand as u32 >> 32 - count & 1) as i32;
        *flags |= (*flags << 11 ^ result >> 20) & FLAG_OVERFLOW;
        return result;
    };
}
#[no_mangle]
pub unsafe fn ror8(dest_operand: i32, mut count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    if 0 == count {
        return dest_operand;
    }
    else {
        count &= 7;
        let result: i32 = dest_operand >> count | dest_operand << 8 - count;
        *flags_changed &= !1 & !FLAG_OVERFLOW;
        *flags = *flags & !1 & !FLAG_OVERFLOW
            | result >> 7 & 1
            | (result << 4 ^ result << 5) & FLAG_OVERFLOW;
        return result;
    };
}
#[no_mangle]
pub unsafe fn ror16(dest_operand: i32, mut count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    if 0 == count {
        return dest_operand;
    }
    else {
        count &= 15;
        let result: i32 = dest_operand >> count | dest_operand << 16 - count;
        *flags_changed &= !1 & !FLAG_OVERFLOW;
        *flags = *flags & !1 & !FLAG_OVERFLOW
            | result >> 15 & 1
            | (result >> 4 ^ result >> 3) & FLAG_OVERFLOW;
        return result;
    };
}
#[no_mangle]
pub unsafe fn ror32(dest_operand: i32, count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    if 0 == count {
        return dest_operand;
    }
    else {
        let result: i32 =
            (dest_operand as u32 >> count | (dest_operand << 32 - count) as u32) as i32;
        *flags_changed &= !1 & !FLAG_OVERFLOW;
        *flags = *flags & !1 & !FLAG_OVERFLOW
            | result >> 31 & 1
            | (result >> 20 ^ result >> 19) & FLAG_OVERFLOW;
        return result;
    };
}
#[no_mangle]
pub unsafe fn rcr8(dest_operand: i32, mut count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    count %= 9;
    if 0 == count {
        return dest_operand;
    }
    else {
        let result: i32 =
            dest_operand >> count | (getcf() as i32) << 8 - count | dest_operand << 9 - count;
        *flags_changed &= !1 & !FLAG_OVERFLOW;
        *flags = *flags & !1 & !FLAG_OVERFLOW
            | result >> 8 & 1
            | (result << 4 ^ result << 5) & FLAG_OVERFLOW;
        return result;
    };
}
#[no_mangle]
pub unsafe fn rcr16(dest_operand: i32, mut count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    count %= 17;
    if 0 == count {
        return dest_operand;
    }
    else {
        let result: i32 =
            dest_operand >> count | (getcf() as i32) << 16 - count | dest_operand << 17 - count;
        *flags_changed &= !1 & !FLAG_OVERFLOW;
        *flags = *flags & !1 & !FLAG_OVERFLOW
            | result >> 16 & 1
            | (result >> 4 ^ result >> 3) & FLAG_OVERFLOW;
        return result;
    };
}
#[no_mangle]
pub unsafe fn rcr32(dest_operand: i32, count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    if 0 == count {
        return dest_operand;
    }
    else {
        let mut result: i32 =
            (dest_operand as u32 >> count | ((getcf() as i32) << 32 - count) as u32) as i32;
        if count > 1 {
            result |= dest_operand << 33 - count
        }
        *flags_changed &= !1 & !FLAG_OVERFLOW;
        *flags = *flags & !1 & !FLAG_OVERFLOW
            | dest_operand >> count - 1 & 1
            | (result >> 20 ^ result >> 19) & FLAG_OVERFLOW;
        return result;
    };
}
#[no_mangle]
pub unsafe fn div8(source_operand: u32) {
    if source_operand == 0 {
        trigger_de();
        return;
    }
    else {
        let target_operand: u16 = *reg16.offset(AX as isize);
        let result: u16 = (target_operand as u32).wrapping_div(source_operand) as u16;
        if result as i32 >= 256 {
            trigger_de();
        }
        else {
            *reg8.offset(AL as isize) = result as u8;
            *reg8.offset(AH as isize) = (target_operand as u32).wrapping_rem(source_operand) as u8
        }
        return;
    };
}

#[no_mangle]
pub unsafe fn idiv8(source_operand: i32) {
    if source_operand == 0 {
        trigger_de();
        return;
    }
    else {
        let target_operand: i32 = *reg16s.offset(AX as isize) as i32;
        let result: i32 = target_operand / source_operand;
        if result >= 128 || result <= -129 {
            trigger_de();
        }
        else {
            *reg8.offset(AL as isize) = result as u8;
            *reg8.offset(AH as isize) = (target_operand % source_operand) as u8
        }
        return;
    };
}
#[no_mangle]
pub unsafe fn div16(source_operand: u32) {
    if source_operand == 0 {
        trigger_de();
        return;
    }
    else {
        let target_operand: u32 =
            (*reg16.offset(AX as isize) as i32 | (*reg16.offset(DX as isize) as i32) << 16) as u32;
        let result: u32 = target_operand.wrapping_div(source_operand);
        if result >= 0x10000 {
            trigger_de();
        }
        else {
            *reg16.offset(AX as isize) = result as u16;
            *reg16.offset(DX as isize) = target_operand.wrapping_rem(source_operand) as u16
        }
        return;
    };
}
#[no_mangle]
pub unsafe fn idiv16(source_operand: i32) {
    if source_operand == 0 {
        trigger_de();
        return;
    }
    else {
        let target_operand: i32 =
            *reg16.offset(AX as isize) as i32 | (*reg16.offset(DX as isize) as i32) << 16;
        let result: i32 = target_operand / source_operand;
        if result >= 32768 || result <= -32769 {
            trigger_de();
        }
        else {
            *reg16.offset(AX as isize) = result as u16;
            *reg16.offset(DX as isize) = (target_operand % source_operand) as u16
        }
        return;
    };
}
#[no_mangle]
pub unsafe fn div32(source_operand: u32) {
    if source_operand == 0 {
        trigger_de();
        return;
    }
    else {
        let target_low: u32 = *reg32s.offset(EAX as isize) as u32;
        let target_high: u32 = *reg32s.offset(EDX as isize) as u32;
        let target_operand: u64 = (target_high as u64) << 32 | target_low as u64;
        let result: u64 = target_operand.wrapping_div(source_operand as u64);
        if result > 4294967295 {
            trigger_de();
            return;
        }
        else {
            let mod_0: i32 = target_operand.wrapping_rem(source_operand as u64) as i32;
            *reg32s.offset(EAX as isize) = result as i32;
            *reg32s.offset(EDX as isize) = mod_0;
            return;
        }
    };
}
#[no_mangle]
pub unsafe fn idiv32(source_operand: i32) {
    if source_operand == 0 {
        trigger_de();
        return;
    }
    else {
        let target_low: u32 = *reg32s.offset(EAX as isize) as u32;
        let target_high: u32 = *reg32s.offset(EDX as isize) as u32;
        let target_operand: i64 = ((target_high as u64) << 32 | target_low as u64) as i64;
        if source_operand == -1 && target_operand == -0x80000000_00000000 as i64 {
            trigger_de();
            return;
        }
        else {
            let result: i64 = target_operand / source_operand as i64;
            if result < -0x80000000 || result > 0x7FFFFFFF {
                trigger_de();
                return;
            }
            else {
                let mod_0: i32 = (target_operand % source_operand as i64) as i32;
                *reg32s.offset(EAX as isize) = result as i32;
                *reg32s.offset(EDX as isize) = mod_0;
                return;
            }
        }
    };
}

#[no_mangle]
pub unsafe fn shl8(dest_operand: i32, count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    if count == 0 {
        return dest_operand;
    }
    else {
        *last_result = dest_operand << count;
        *last_op_size = OPSIZE_8;
        *flags_changed = FLAGS_ALL & !1 & !FLAG_OVERFLOW;
        *flags = *flags & !1 & !FLAG_OVERFLOW
            | *last_result >> 8 & 1
            | (*last_result << 3 ^ *last_result << 4) & FLAG_OVERFLOW;
        return *last_result;
    };
}
#[no_mangle]
pub unsafe fn shl16(dest_operand: i32, count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    if count == 0 {
        return dest_operand;
    }
    else {
        *last_result = dest_operand << count;
        *last_op_size = OPSIZE_16;
        *flags_changed = FLAGS_ALL & !1 & !FLAG_OVERFLOW;
        *flags = *flags & !1 & !FLAG_OVERFLOW
            | *last_result >> 16 & 1
            | (*last_result >> 5 ^ *last_result >> 4) & FLAG_OVERFLOW;
        return *last_result;
    };
}
#[no_mangle]
pub unsafe fn shl32(dest_operand: i32, count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    if count == 0 {
        return dest_operand;
    }
    else {
        *last_result = dest_operand << count;
        *last_op_size = OPSIZE_32;
        *flags_changed = FLAGS_ALL & !1 & !FLAG_OVERFLOW;
        // test this
        *flags = *flags & !1 & !FLAG_OVERFLOW | dest_operand >> 32 - count & 1;
        *flags |= (*flags & 1 ^ *last_result >> 31 & 1) << 11 & FLAG_OVERFLOW;
        return *last_result;
    };
}
#[no_mangle]
pub unsafe fn shr8(dest_operand: i32, count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    if count == 0 {
        return dest_operand;
    }
    else {
        *last_result = dest_operand >> count;
        *last_op_size = OPSIZE_8;
        *flags_changed = FLAGS_ALL & !1 & !FLAG_OVERFLOW;
        *flags = *flags & !1 & !FLAG_OVERFLOW
            | dest_operand >> count - 1 & 1
            | (dest_operand >> 7 & 1) << 11 & FLAG_OVERFLOW;
        return *last_result;
    };
}
#[no_mangle]
pub unsafe fn shr16(dest_operand: i32, count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    if count == 0 {
        return dest_operand;
    }
    else {
        *last_result = dest_operand >> count;
        *last_op_size = OPSIZE_16;
        *flags_changed = FLAGS_ALL & !1 & !FLAG_OVERFLOW;
        *flags = *flags & !1 & !FLAG_OVERFLOW
            | dest_operand >> count - 1 & 1
            | dest_operand >> 4 & FLAG_OVERFLOW;
        return *last_result;
    };
}
#[no_mangle]
pub unsafe fn shr32(dest_operand: i32, count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    if count == 0 {
        return dest_operand;
    }
    else {
        *last_result = (dest_operand as u32 >> count) as i32;
        *last_op_size = OPSIZE_32;
        *flags_changed = FLAGS_ALL & !1 & !FLAG_OVERFLOW;
        *flags = ((*flags & !1 & !FLAG_OVERFLOW) as u32
            | dest_operand as u32 >> count - 1 & 1
            | (dest_operand >> 20 & FLAG_OVERFLOW) as u32) as i32;
        return *last_result;
    };
}
#[no_mangle]
pub unsafe fn sar8(dest_operand: i32, count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    if count == 0 {
        return dest_operand;
    }
    else {
        if count < 8 {
            *last_result = dest_operand << 24 >> count + 24;
            // of is zero
            *flags = *flags & !1 & !FLAG_OVERFLOW | dest_operand >> count - 1 & 1
        }
        else {
            *last_result = dest_operand << 24 >> 31;
            *flags = *flags & !1 & !FLAG_OVERFLOW | *last_result & 1
        }
        *last_op_size = OPSIZE_8;
        *flags_changed = FLAGS_ALL & !1 & !FLAG_OVERFLOW;
        return *last_result;
    };
}
#[no_mangle]
pub unsafe fn sar16(dest_operand: i32, count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    if count == 0 {
        return dest_operand;
    }
    else {
        if count < 16 {
            *last_result = dest_operand << 16 >> count + 16;
            *flags = *flags & !1 & !FLAG_OVERFLOW | dest_operand >> count - 1 & 1
        }
        else {
            *last_result = dest_operand << 16 >> 31;
            *flags = *flags & !1 & !FLAG_OVERFLOW | *last_result & 1
        }
        *last_op_size = OPSIZE_16;
        *flags_changed = FLAGS_ALL & !1 & !FLAG_OVERFLOW;
        return *last_result;
    };
}
#[no_mangle]
pub unsafe fn sar32(dest_operand: i32, count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    if count == 0 {
        return dest_operand;
    }
    else {
        *last_result = dest_operand >> count;
        *last_op_size = OPSIZE_32;
        *flags_changed = FLAGS_ALL & !1 & !FLAG_OVERFLOW;
        *flags =
            ((*flags & !1 & !FLAG_OVERFLOW) as u32 | dest_operand as u32 >> count - 1 & 1) as i32;
        return *last_result;
    };
}

#[no_mangle]
pub unsafe fn shrd16(dest_operand: i32, source_operand: i32, count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    if count == 0 {
        return dest_operand;
    }
    else {
        if count <= 16 {
            *last_result = dest_operand >> count | source_operand << 16 - count;
            *flags = *flags & !1 | dest_operand >> count - 1 & 1
        }
        else {
            *last_result = dest_operand << 32 - count | source_operand >> count - 16;
            *flags = *flags & !1 | source_operand >> count - 17 & 1
        }
        *last_op_size = OPSIZE_16;
        *flags_changed = FLAGS_ALL & !1 & !FLAG_OVERFLOW;
        *flags = *flags & !FLAG_OVERFLOW | (*last_result ^ dest_operand) >> 4 & FLAG_OVERFLOW;
        return *last_result;
    };
}
#[no_mangle]
pub unsafe fn shrd32(dest_operand: i32, source_operand: i32, count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    if count == 0 {
        return dest_operand;
    }
    else {
        *last_result =
            (dest_operand as u32 >> count | (source_operand << 32 - count) as u32) as i32;
        *last_op_size = OPSIZE_32;
        *flags_changed = FLAGS_ALL & !1 & !FLAG_OVERFLOW;
        *flags = ((*flags & !1) as u32 | dest_operand as u32 >> count - 1 & 1) as i32;
        *flags = *flags & !FLAG_OVERFLOW | (*last_result ^ dest_operand) >> 20 & FLAG_OVERFLOW;
        return *last_result;
    };
}
#[no_mangle]
pub unsafe fn shld16(dest_operand: i32, source_operand: i32, count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    if count == 0 {
        return dest_operand;
    }
    else {
        if count <= 16 {
            *last_result =
                ((dest_operand << count) as u32 | source_operand as u32 >> 16 - count) as i32;
            *flags = ((*flags & !1) as u32 | dest_operand as u32 >> 16 - count & 1) as i32
        }
        else {
            *last_result = dest_operand >> 32 - count | source_operand << count - 16;
            *flags = ((*flags & !1) as u32 | source_operand as u32 >> 32 - count & 1) as i32
        }
        *last_op_size = OPSIZE_16;
        *flags_changed = FLAGS_ALL & !1 & !FLAG_OVERFLOW;
        *flags = *flags & !FLAG_OVERFLOW | (*flags & 1 ^ *last_result >> 15 & 1) << 11;
        return *last_result;
    };
}
#[no_mangle]
pub unsafe fn shld32(dest_operand: i32, source_operand: i32, count: i32) -> i32 {
    dbg_assert!(count >= 0 && count < 32);
    if count == 0 {
        return dest_operand;
    }
    else {
        *last_result =
            ((dest_operand << count) as u32 | source_operand as u32 >> 32 - count) as i32;
        *last_op_size = OPSIZE_32;
        *flags_changed = FLAGS_ALL & !1 & !FLAG_OVERFLOW;
        *flags = ((*flags & !1) as u32 | dest_operand as u32 >> 32 - count & 1) as i32;
        if count == 1 {
            *flags = *flags & !FLAG_OVERFLOW | (*flags & 1 ^ *last_result >> 31 & 1) << 11
        }
        else {
            *flags &= !FLAG_OVERFLOW
        }
        return *last_result;
    };
}

#[no_mangle]
pub unsafe fn bt_reg(bit_base: i32, bit_offset: i32) {
    *flags = *flags & !1 | bit_base >> bit_offset & 1;
    *flags_changed &= !1;
}
#[no_mangle]
pub unsafe fn btc_reg(bit_base: i32, bit_offset: i32) -> i32 {
    *flags = *flags & !1 | bit_base >> bit_offset & 1;
    *flags_changed &= !1;
    return bit_base ^ 1 << bit_offset;
}
#[no_mangle]
pub unsafe fn bts_reg(bit_base: i32, bit_offset: i32) -> i32 {
    *flags = *flags & !1 | bit_base >> bit_offset & 1;
    *flags_changed &= !1;
    return bit_base | 1 << bit_offset;
}
#[no_mangle]
pub unsafe fn btr_reg(bit_base: i32, bit_offset: i32) -> i32 {
    *flags = *flags & !1 | bit_base >> bit_offset & 1;
    *flags_changed &= !1;
    return bit_base & !(1 << bit_offset);
}
#[no_mangle]
pub unsafe fn bt_mem(virt_addr: i32, mut bit_offset: i32) {
    let bit_base: i32 = return_on_pagefault!(safe_read8(virt_addr + (bit_offset >> 3)));
    bit_offset &= 7;
    *flags = *flags & !1 | bit_base >> bit_offset & 1;
    *flags_changed &= !1;
}
#[no_mangle]
pub unsafe fn btc_mem(virt_addr: i32, mut bit_offset: i32) {
    let phys_addr = return_on_pagefault!(translate_address_write(virt_addr + (bit_offset >> 3)));
    let bit_base: i32 = read8(phys_addr);
    bit_offset &= 7;
    *flags = *flags & !1 | bit_base >> bit_offset & 1;
    *flags_changed &= !1;
    write8(phys_addr, bit_base ^ 1 << bit_offset);
}
#[no_mangle]
pub unsafe fn btr_mem(virt_addr: i32, mut bit_offset: i32) {
    let phys_addr = return_on_pagefault!(translate_address_write(virt_addr + (bit_offset >> 3)));
    let bit_base: i32 = read8(phys_addr);
    bit_offset &= 7;
    *flags = *flags & !1 | bit_base >> bit_offset & 1;
    *flags_changed &= !1;
    write8(phys_addr, bit_base & !(1 << bit_offset));
}
#[no_mangle]
pub unsafe fn bts_mem(virt_addr: i32, mut bit_offset: i32) {
    let phys_addr = return_on_pagefault!(translate_address_write(virt_addr + (bit_offset >> 3)));
    let bit_base: i32 = read8(phys_addr);
    bit_offset &= 7;
    *flags = *flags & !1 | bit_base >> bit_offset & 1;
    *flags_changed &= !1;
    write8(phys_addr, bit_base | 1 << bit_offset);
}
#[no_mangle]
pub unsafe fn bsf16(old: i32, bit_base: i32) -> i32 {
    *flags_changed = FLAGS_ALL & !FLAG_ZERO;
    *last_op_size = OPSIZE_16;
    if bit_base == 0 {
        *flags |= FLAG_ZERO;
        *last_result = bit_base;
        // not defined in the docs, but value doesn't change on my intel machine
        return old;
    }
    else {
        *flags &= !FLAG_ZERO;
        *last_result = int_log2(-bit_base & bit_base);
        return *last_result;
    };
}
#[no_mangle]
pub unsafe fn bsf32(old: i32, bit_base: i32) -> i32 {
    *flags_changed = FLAGS_ALL & !FLAG_ZERO;
    *last_op_size = OPSIZE_32;
    if bit_base == 0 {
        *flags |= FLAG_ZERO;
        *last_result = bit_base;
        return old;
    }
    else {
        *flags &= !FLAG_ZERO;
        *last_result = int_log2((-bit_base & bit_base) as u32 as i32);
        return *last_result;
    };
}
#[no_mangle]
pub unsafe fn bsr16(old: i32, bit_base: i32) -> i32 {
    *flags_changed = FLAGS_ALL & !FLAG_ZERO;
    *last_op_size = OPSIZE_16;
    if bit_base == 0 {
        *flags |= FLAG_ZERO;
        *last_result = bit_base;
        return old;
    }
    else {
        *flags &= !FLAG_ZERO;
        *last_result = int_log2(bit_base);
        return *last_result;
    };
}
#[no_mangle]
pub unsafe fn bsr32(old: i32, bit_base: i32) -> i32 {
    *flags_changed = FLAGS_ALL & !FLAG_ZERO;
    *last_op_size = OPSIZE_32;
    if bit_base == 0 {
        *flags |= FLAG_ZERO;
        *last_result = bit_base;
        return old;
    }
    else {
        *flags &= !FLAG_ZERO;
        *last_result = int_log2(bit_base as u32 as i32);
        return *last_result;
    };
}
#[no_mangle]
pub unsafe fn popcnt(v: i32) -> i32 {
    *flags_changed = 0;
    *flags &= !FLAGS_ALL;
    if 0 != v {
        return v.count_ones() as i32;
    }
    else {
        *flags |= FLAG_ZERO;
        return 0;
    };
}
#[no_mangle]
pub unsafe fn saturate_sw_to_ub(v: u32) -> u32 {
    dbg_assert!(v & 4294901760 == 0);
    let mut ret: u32 = v;
    if ret >= 32768 {
        ret = 0
    }
    else if ret > 255 {
        ret = 255
    }
    dbg_assert!(ret & 4294967040 == 0);
    return ret;
}
#[no_mangle]
pub unsafe fn saturate_sw_to_sb(v: i32) -> i32 {
    dbg_assert!(v as u32 & 4294901760 == 0);
    let mut ret: i32 = v;
    if ret > 65408 {
        ret = ret & 255
    }
    else if ret > 32767 {
        ret = 128
    }
    else if ret > 127 {
        ret = 127
    }
    dbg_assert!(ret as u32 & 4294967040 == 0);
    return ret;
}
#[no_mangle]
pub unsafe fn saturate_sd_to_sw(v: u32) -> u32 {
    let mut ret: u32 = v;
    if ret > 4294934528 {
        ret = ret & 0xFFFF
    }
    else if ret > 0x7FFFFFFF {
        ret = 32768
    }
    else if ret > 32767 {
        ret = 32767
    }
    dbg_assert!(ret & 4294901760 == 0);
    return ret;
}
#[no_mangle]
pub unsafe fn saturate_sd_to_sb(v: u32) -> u32 {
    let mut ret: u32 = v;
    if ret > 4294967168 {
        ret = ret & 255
    }
    else if ret > 0x7FFFFFFF {
        ret = 128
    }
    else if ret > 127 {
        ret = 127
    }
    dbg_assert!(ret & 4294967040 == 0);
    return ret;
}
#[no_mangle]
pub unsafe fn saturate_sd_to_ub(v: i32) -> i32 {
    let mut ret: i32 = v;
    if ret < 0 {
        ret = 0
    }
    dbg_assert!(ret as u32 & 4294967040 == 0);
    return ret;
}
#[no_mangle]
pub unsafe fn saturate_ud_to_ub(v: u32) -> u32 {
    let mut ret: u32 = v;
    if ret > 255 {
        ret = 255
    }
    dbg_assert!(ret & 4294967040 == 0);
    return ret;
}
#[no_mangle]
pub unsafe fn saturate_uw(v: u32) -> i32 {
    let mut ret: u32 = v;
    if ret > 0x7FFFFFFF {
        ret = 0
    }
    else if ret > 0xFFFF {
        ret = 0xFFFF
    }
    dbg_assert!(ret & 4294901760 == 0);
    return ret as i32;
}

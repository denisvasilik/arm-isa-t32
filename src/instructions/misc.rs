#[inline]
pub unsafe fn bkpt(id: usize, nr: usize, arg: usize) -> usize {
    let mut asm_nr = nr;
    asm!("bkpt 0xAB" : "+{r0}"(asm_nr) : "{r1}"(arg) :: "volatile");
    asm_nr
}

int main()
{
	int count = rust_func();
	int i;
	asm("mov $0, %eax");
	for(i=0;i<count;i++){
		asm("inc %rax");
	}
	while (1);
}

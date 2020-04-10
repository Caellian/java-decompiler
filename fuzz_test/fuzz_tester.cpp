
extern "C" int LLVMFuzzerTestOneInput(int a)
{
  long b = ++a;
  if (b > 10)
  {
    b += 1;
  }
  return 0;
}

#include <iostream>
using namespace std;

int main()
{
  int a, b;
  cin >> a >> b;
  if (0 <= a && a <= 100 && 0 <= b && b <= 100)
  {
    cout << a + b << endl;
  }
  else
  {
    cout << "Condition not met" << endl;
  }
}
#include <iostream>


using namespace std;

int solve(int a, int b, int c, int x, int y, int z) {
  return (a +b) * x + (b + c) * y + (c + a) * z;
}


int main(void){
  int a, b, c;
  int x, y, z;
  cin >> a >> b >> c >> x >> y >> z;
  int ma = 1e9;

  ma = min(ma, solve(a, b, c, x, y, z));
  ma = min(ma, solve(a, b, c, x, z, y));
  ma = min(ma, solve(a, b, c, y, x, z));
  ma = min(ma, solve(a, b, c, y, z, x));
  ma = min(ma, solve(a, b, c, z, x, y));
  ma = min(ma, solve(a, b, c, z, y, x));

  cout << ma *2 << endl;
}

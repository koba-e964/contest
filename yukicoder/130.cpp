#include <iostream>
#include <algorithm>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;

int a[100100];
const int inf = 0x7ff00000;
int rec(int s, int t, int b) {
  if (b < 0) {
    return 0;
  }
  if (s >= t) {
    return inf;
  }
  if ((a[s] & (1 << b)) == (a[t - 1] & (1 << b))) {
    return rec(s, t, b - 1);
  }
  int i = s;
  while (i < t) {
    if (a[i] & (1 << b)) {
      break;
    }
    i ++;
  }
  // [s,i), [i,t)
  int r1 = rec(s, i, b - 1);
  int r2 = rec(i, t, b - 1);
  return min(r1, r2) + (1 << b);
}
int main(void){
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> a[i];
  }
  sort(a, a + n);
  cout << rec(0, n, 30) << endl;
}

#include <iostream>
#include <algorithm>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef pair<int, int> PI;
const double EPS=1e-9;

const int N = 10;

int n;
int a[N], b[N];
PI dp[1 << N];
int main(void){
  int m;
  cin >> n;
  REP(i, 0, n) {
    cin >> a[i];
  }
  cin >> m;
  REP(i, 0, m) {
    cin >> b[i];
  }
  sort(b, b + m);
  reverse(b, b + m);
  dp[0] = PI(0, 0);
  REP(bits, 1, 1 << n) {
    PI ma = PI(m + 1, 0);
    REP(i, 0, n) {
      if ((bits & (1 << i)) == 0) {
	continue;
      }
      PI res = dp[bits ^ 1 << i];
      if (res.first >= m) {
	continue;
      }
      int k = res.first;
      if (res.second + a[i] > b[k]) {
	if (b[k + 1] >= a[i]) {
	  ma = min(ma, PI(res.first + 1, a[i]));
	} else {
	  continue;
	}
      } else {
	ma = min(ma, PI(res.first, res.second + a[i]));
      }
    }
    dp[bits] = ma;
  }
  PI res = dp[(1 << n) - 1]; 
  cout << (res.first >= m + 1 ? -1 : (res.first + (res.second ? 1 : 0))) << endl;
  return 0;
}

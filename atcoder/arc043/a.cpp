#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;

const int N = 100010;
double s[N];

int main(void){
  int n, a, b;
  cin >> n >> a >> b;
  double avr = 0, ma = 0, mi = 1e10;
  REP(i, 0, n) {
    cin >> s[i];
    avr += s[i];
    ma = max(ma, s[i]);
    mi = min(mi, s[i]);
  }
  avr /= n;
  double p = b / (ma - mi);
  double q = - p * avr + a;
  if (mi == ma) {
    cout << -1 << endl;
  } else {
    printf("%.9f %.9f\n", p, q);
  }
}

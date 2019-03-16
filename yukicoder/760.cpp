#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>
#include <complex>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

typedef complex<double> comp;

int main(void) {
  double pi = acos(-1);
  ios::sync_with_stdio(false);
  cin.tie(0);
  double xa, ya, ta;
  cin >> xa >> ya >> ta;
  ta *= pi / 180;
  comp a(xa, ya);
  cin >> xa >> ya;
  comp p11(xa, ya);
  cin >> xa >> ya;
  comp p12(xa, ya);
  cin >> xa >> ya;
  comp p21(xa, ya);
  cin >> xa >> ya;
  comp p22(xa, ya);
  comp rel1 = (p21 - a) * polar(1.0, -ta);
  comp rel2 = (p22 - a) * polar(1.0, -ta);
  double theta = arg(p12 - p11);
  double tb = theta - arg(rel2 - rel1);
  comp b = p11 - polar(1.0, tb) * rel1;
  cout << setprecision(15) << b.real() << " " << b.imag() << " "
       << tb * 180 / pi << endl;
}

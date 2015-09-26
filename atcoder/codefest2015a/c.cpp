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



int main(void){
  int n;
  ll t;
  cin >> n >> t;
  vector<ll> a(n), b(n);
  REP(i, 0, n) {
    cin >> a[i] >> b[i];
    t -= b[i];
    a[i] -= b[i];
    b[i] = 0;
  }
  sort(a.begin(), a.end());
  int c = 0;
  if (t < 0) {
    cout << -1 << endl;
    return 0;
  }
  REP(i, 0, n) {
    if (t >= a[i]) {
      t -= a[i];
      c++;
    } else {
      break;
    }
  }
  cout << n - c << endl;

}

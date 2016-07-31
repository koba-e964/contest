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
  ll l;
  cin >> n >> l;
  vector<ll> a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }

  // exists consecutive ropes r_i, r_{i + 1}. r_i + r_{i + 1} >= l ?
  bool ok = 0;
  int pos = -1;
  REP(i, 0, n - 1) {
    if (a[i] + a[i + 1] >= l) {
      ok = 1;
      pos = i;
    }
  }
  if (!ok) {
    cout << "Impossible" << endl;
    return 0;
  }
  cout << "Possible" << endl;
  REP(i, 0, pos) {
    cout << i + 1 << endl;
  }
  for (int i = n - 2; i > pos; --i) {
    cout << i + 1 << endl;
  }
  cout << pos + 1 << endl;
}

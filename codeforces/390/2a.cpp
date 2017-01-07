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
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void){
  int n;
  cin >> n;
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  bool all_zero = true;
  int tot = 0;
  REP(i, 0, n) {
    tot += a[i];
    all_zero &= a[i] == 0;
  }
  if (all_zero) {
    cout << "NO" << endl;
    return 0;
  }
  cout << "YES" << endl;
  if (tot != 0) {
    cout << 1 << endl;
    cout << 1 << " " << n << endl;
    return 0;
  }
  int idx = -1; // the index of the first non-zero element
  REP(i, 0, n) {
    if (a[i] != 0) {
      idx = i;
      break;
    }
  }
  assert (idx >= 0);
  cout << 2 << endl;
  cout << 1 << " " << idx + 1 << endl;
  cout << idx + 2 << " " << n << endl;
}

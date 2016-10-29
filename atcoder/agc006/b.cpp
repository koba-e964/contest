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
  int n, x;
  cin >> n >> x;
  if (n == 2) {
    if (x == 2) {
      cout << "Yes" << endl;
      cout << 1 << endl << 2 << endl << 3 << endl;
    } else {
      cout << "No" << endl;
    }
    return 0;
  }
  if (x == 1 || x == 2 * n - 1) {
    cout << "No" << endl;
    return 0;
  }
  cout << "Yes" << endl;
  // ... 3 5 2 1 4
  // ... (2n-3) (2n-1) x 1 (2n-2)
  VI pool;
  pool.reserve(2 * n - 6);
  VI targ(5);
  if (n == 3 && x == 3) {
    REP(i, 0, 5) targ[i] = i + 1;
  } else if (x < 2 * n - 3) {
    REP(i, 2, 2 * n - 3) {
      if (i != x) pool.push_back(i);
    }
    targ[0] = 2*n-3;
    targ[1] = 2*n-1;
    targ[2] = x;
    targ[3] = 1;
    targ[4] = 2*n-2;
  } else {
    assert (x >= 4);
    // ... 3 1 x (2n-1) 2
    REP(i, 4, 2 * n - 1) {
      if (i != x) pool.push_back(i);
    }
    targ[0] = 3;
    targ[1] = 1;
    targ[2] = x;
    targ[3] = 2 * n - 1;
    targ[4] = 2;
  }
  REP(i, 0, n - 3) {
    cout << pool[i] << endl;
  }
  REP(i, 0, 5) {
    cout << targ[i] << endl;
  }
  REP(i, n - 3, 2 * n - 6) {
    cout << pool[i] << endl;
  }
}

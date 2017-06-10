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


int main(void){
  int n, k;
  cin >> n >> k;
  VI a(n), b(k);
  REP(i, 0, n) {
    cin >> a[i];
  }
  REP(i, 0, k) {
    cin >> b[i];
  }
  sort(b.rbegin(), b.rend());
  int pos = 0;
  REP(i, 0, n) {
    if (a[i] == 0) {
      a[i] = b[pos];
      pos += 1;
    }
  }
  bool ok = false;
  REP(i, 0, n - 1) {
    if (a[i] > a[i + 1]) {
      ok = true;
      break;
    }
  }
  cout << (ok ? "Yes" : "No") << endl;
}

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

const int Q = 100100;
int a[Q], b[Q];

int main(void){
  int n, q;
  cin >> n >> q;
  REP(i, 0, q) {
    cin >> a[i] >> b[i];
    a[i]--, b[i]--;
  }
  VI t(n, 0);
  t[0] = 1;
  int pos = 0;
  REP(i, 0, q + 1) {
    if (pos >= 1) {
      t[pos - 1] = 1;
    }
    if (pos < n - 1) {
      t[pos + 1] = 1;
    }
    if (i < q) {
      swap(t[a[i]], t[b[i]]);
      if (pos == a[i]) {
	pos = b[i];
      } else if (pos == b[i]) {
	pos = a[i];
      }
    }
  }
  int cnt = 0;
  REP(i, 0, n) cnt += t[i] >= 1;
  cout << cnt << endl;
}

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

const int DEBUG = 0;

int main(void){
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  ll sum = 0;
  ll ma = 0;
  REP(i, 0, n) {
    ll quo = (a[i] - 1) / (ma + 1);
    sum += quo;
    a[i] -= quo * (ma + 1);
    if (quo >= 1) {
      a[i] = 1;
    }
    ma = max(ma, a[i]);
  }
  if (DEBUG) {
    REP(i, 0, n) {
      cerr << "a[" << i << "]=" << a[i] << endl;
    }
  }
  cout << sum << endl;
}

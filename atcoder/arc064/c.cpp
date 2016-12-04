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
  ll x;
  cin >> n >> x;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  ll tot = 0;
  REP(i, 0, n) {
    if (a[i] > x) {
      tot += a[i] - x;
      a[i] = x;
    }
  }
  REP(i, 0, n - 1) {
    if (a[i] + a[i + 1] > x) {
      ll d = a[i] + a[i + 1] - x;
      tot += d;
      a[i + 1] -= d;
    }
  }
  cout << tot << endl;
}

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


const int N = 200100;
const ll inf = 3e14;

int DEBUG = 0;

ll fl[N], fr[N];

int main(void){
  ll a, b, c;
  cin >> a >> b >> c;
  string t;
  cin >> t;
  int n = t.length();
  ll mi = inf;
  fl[0] = 0;
  REP(i, 1, n + 1) {
    if (i >= 2 ? t[i - 1] != t[i - 2] : t[i - 1] == '1') {
      fl[i] = fl[i - 1] + 1;
    } else {
      fl[i] = fl[i - 1];
    }
  }
  fr[n] = 0;
  for (int i = n - 1; i >= 0; --i) {
    if (i <= n-2 ? t[i] != t[i + 1] : t[i] == '0') {
      fr[i] = fr[i + 1] + 1;
    } else {
      fr[i] = fr[i + 1];
    }
  }
  if (DEBUG) {
    REP(i, 0, n) {
      cerr << "fl[" << i << "]=" << fl[i] << "," << "fr[" << i << "]=" << fr[i] << endl;
    }
  }
  REP(i, 0, n + 1) {
    ll cost = a * i + b * (n - i);
    cost += c * max(fl[i], fr[i]);
    mi = min(mi, cost);
  }
  cout << mi << endl;
}

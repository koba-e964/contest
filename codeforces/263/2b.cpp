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

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  ll k;
  cin >> n >> k;
  string s;
  cin >> s;
  VI freq(26);
  REP(i, 0, n) {
    freq[s[i] - 'A'] += 1;
  }
  sort(freq.rbegin(), freq.rend());
  ll tot = 0;
  REP(i, 0, 26) {
    ll q = min(k, (ll)freq[i]);
    tot += q * q;
    k -= q;
  }
  cout << tot << "\n";
}

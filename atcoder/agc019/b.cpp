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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
  string s;
  cin >> s;
  int n = s.length();
  ll tot = 1;
  VL freq(26);
  REP(i, 0, n) {
    freq[s[i] - 'a'] += 1;
  }
  REP(i, 0, 26) {
    REP(j, i + 1, 26) {
      tot += freq[i] * freq[j];
    }
  }
  cout << tot << endl;
}

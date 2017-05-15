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
  string s, t;
  cin >> s >> t;
  int n = s.length();
  int sf[26] = {};
  int tf[26] = {};
  REP(i, 0, n) {
    sf[s[i] - 'a'] += 1;
  }
  REP(i, 0, n) {
    tf[t[i] - 'a'] += 1;
  }
  int spos = 0;
  int tpos = 25;
  string res(n, '+');
  int turn = 0;
  for (; turn < n; ++turn) {
    while (spos < 26 && sf[spos] == 0) spos++;
    while (tpos >= 0 && tf[tpos] == 0) tpos--;
    if (spos >= tpos) { break; } 
    if (turn % 2 == 0) {
      res[turn] = 'a' + spos;
      sf[spos] -= 1;
    } else {
      res[turn] = 'a' + tpos;
      tf[tpos] -= 1;
    }
  }
  // Use a different tact.
  int cturn = turn;
  for (; turn < n; ++turn) {
    while (spos < 26 && sf[spos] == 0) spos++;
    while (tpos >= 0 && tf[tpos] == 0) tpos--;
    if (turn % 2 == 0) {
      res[turn] = 'a' + spos;
      sf[spos] -= 1;
    } else {
      res[turn] = 'a' + tpos;
      tf[tpos] -= 1;
    }
  }
  if ((n - cturn) % 2 == 0) {
    for (int i = cturn; i < n; i += 2) {
      swap(res[i], res[i + 1]);
    }
  }
  cout << res << endl;
}

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


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string good, pat;
  int n;
  cin >> good >> pat >> n;
  vector<bool> boni(26, false);
  REP(i, 0, good.length()) {
    boni[good[i] - 'a'] = true;
  }
  int wild = -1;
  REP(i, 0, pat.length()) {
    if (pat[i] == '*') {
      wild = i;
      break;
    }
  }
  REP(i, 0, n) {
    string s;
    cin >> s;
    if (wild == -1 && s.length() != pat.length()) {
      cout << "NO\n";
      continue;
    }
    if (wild == -1) {
      bool ok = true;
      REP(i, 0, s.length()) {
	if (pat[i] == '?') {
	  ok &= boni[s[i] - 'a'];
	} else {
	  ok &= pat[i] == s[i];
	}
      }
      cout << (ok ? "YES" : "NO") << "\n";
      continue;
    }
    if (s.length() < pat.length() - 1) {
      cout << "NO\n";
      continue;
    }
    int diff = (int) s.length() - (int) pat.length();
    bool ok = true;
    REP(i, 0, wild) {
      if (pat[i] == '?') {
	ok &= boni[s[i] - 'a'];
      } else {
	ok &= pat[i] == s[i];
      }
    }
    REP(i, wild, wild + diff + 1) {
      ok &= not boni[s[i] - 'a'];
    }
    REP(i, wild + diff + 1, s.length()) {
      if (pat[i - diff] == '?') {
	ok &= boni[s[i] - 'a'];
      } else {
	ok &= pat[i - diff] == s[i];
      }
    }
    cout << (ok ? "YES" : "NO") << "\n";

  }
}

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

bool unsym(char a, char b) {
  map<char, char> t;
  t['i'] = 'i';
  t['w'] = 'w';
  t['('] = ')';
  t[')'] = '(';
  return t[a] != b;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string s;
  cin >> s;
  int cnt = 0;
  REP(i, 0, (s.length() + 1) / 2) {
    if (unsym(s[i], s[s.length() - 1 - i])) {
      cnt += 1;
    }
  }
  cout << cnt << "\n";
}

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

void fail(void) {
  cout << -1 << endl;
  exit(0);
}

int n;
VI ops;
string s, t;
string aux;


void shift(int x) {
  rotate(s.begin(), s.begin() + n - x, s.end());
  reverse(s.begin(), s.begin() + x);
  ops.push_back(x);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n >> s >> t;
  VI freq(26);
  REP(i, 0, n) {
    freq[s[i] - 'a'] += 1;
    freq[t[i] - 'a'] -= 1;
  }
  REP(i, 0, 26) {
    if (freq[i] != 0) {
      fail();
    }
  }
  aux = string(n, '*');
  REP(i, 0, n) {
    if (i % 2 == 0) {
      aux[i] = t[n - 1 - i / 2];
    } else {
      aux[i] = t[i / 2];
    }
  }
  reverse(aux.begin(), aux.end());
  // TODO uses 4n queries
  REP(i, 0, n) {
    int idx = -1;
    REP(j, i, n) {
      if (s[j] == aux[i]) {
	idx = j;
	break;
      }
    }
    assert (idx >= 0);
    shift(n - idx - 1);
    shift(idx - i + 1);
    shift(i);
  }
  assert (s == t);
  cout << ops.size() << "\n";
  REP(i, 0, ops.size()) {
    cout << ops[i] << (i == (int) ops.size() - 1 ? "" : " ");
  }
  cout << "\n";
}

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
  cin.tie(0);
  ios::sync_with_stdio(false);
  int n, m;
  cin >> n >> m;
  string s, t;
  cin >> s >> t;
  int mi = 1e8;
  int mini = -1;
  REP(i, 0, m - n + 1) {
    int diff = 0;
    REP(j, 0, n) {
      if (s[j] != t[i + j]) {
	diff += 1;
      }
    }
    if (mi > diff) {
      mi = diff;
      mini = i;
    }
  }
  cout << mi << "\n";
  VI diffs;
  REP(i, 0, n) {
    if (s[i] != t[mini + i]) {
      diffs.push_back(i + 1);
    }
  }
  REP(i, 0, diffs.size()) {
    cout << diffs[i] << (i == diffs.size() - 1 ? "\n" : " ");
  }
}

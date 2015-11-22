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
typedef pair<int, int> PI;
const double EPS=1e-9;


vector<string> solve(const vector<string> &v, const vector<string> &w, const VI& len) {
  int n = v.size();
  int k = len.size();
  vector<string> ret(k);
  REP(i, 0, n) {
    int pos = 0;
    REP(j, 0, v[i].length()) {
      int d = v[i][j] - '1';
      if (pos + len[d] > w[i].length()) {
	return vector<string>();
      }
      string slice = w[i].substr(pos, len[d]);
      ret[d] = slice;
      pos += len[d];
    }
    if (pos != w[i].length()) {
      return vector<string>();
    }
  }
  REP(i, 0, n) {
    string g;
    REP(j, 0, v[i].length()) {
      g = g + ret[v[i][j] - '1'];
    }
    if (g != w[i]) {
      return vector<string>();
    }
  }
  return ret;
}

int main(void){
  int k, n;
  cin >> k >> n;
  vector<string> v(n), w(n);
  REP(i, 0, n) {
    cin >> v[i] >> w[i];
  }
  REP(i, 0, pow(3, k)) {
    VI len(k);
    int c = i;
    REP(t, 0, k) {
      len[t] = c % 3 + 1;
      c /= 3;
    }
    vector<string> ret = solve(v, w, len);
    if (ret.size() > 0) {
      REP(t, 0, k) {
	cout << ret[t] << endl;
      }
      return 0;
    }
  }
  assert (0);

}

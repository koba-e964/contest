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

const int N = 101;

string s[N], e[N];
int st[N], et[N];

int parse(string s) {
  stringstream ss;
  ss << s;
  int h, m, sec , mm;
  ss >> h;
  ss.ignore();
  ss >> m;
  ss.ignore();
  ss >> sec;
  ss.ignore();
  ss >> mm;
  return h * 3600000 + m * 60000 + sec * 1000 + mm;
}

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> s[i] >> e[i];
    st[i] = parse(s[i]);
    et[i] = parse(e[i]);
  }
  int lo = 0;
  int hi = 86400000;
  REP(i, 0, n) {
    if (st[i] >= et[i]) {
      lo = max(lo, st[i]);
      hi = min(hi, et[i] + 1000);
    }
  }
  assert (lo < hi);
  REP(i, 0, n) {
    if (st[i] >= et[i]) {
      cout << et[i] - st[i] + 1000 << endl;
    } else 
    if (lo - 1000 < st[i] && st[i] < hi) {
      cout << -1 << endl;
    } else if (lo - 1000 < et[i] && et[i] < hi) {
      cout << -1 << endl;
    } else
    if (st[i] >= et[i]) {
      cout << et[i] - st[i] + 1000 << endl;
    } else if (hi <= st[i] || et[i] <= lo) {
      cout << et[i] - st[i] << endl;
    } else if (st[i] <= lo - 1000 && et[i] >= hi) {
      cout << et[i] - st[i] + 1000 << endl;
    } else {
      cout << -1 << endl;
    }
  }
}

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

int n, s;
const int N = 33;
int p[N];

const int W = 0x10000;
PI fi[W], se[W];

struct Comp {
  bool operator() (PI a, PI b) {
    return a.first < b.first;
  }
};

unsigned int flip(unsigned int v) {
  v = ((v & 0xaaaaaaaa) >> 1) | ((v & 0x55555555) << 1);
  v = ((v & 0xcccccccc) >> 2) | ((v & 0x33333333) << 2);
  v = ((v & 0xf0f0f0f0) >> 4) | ((v & 0x0f0f0f0f) << 4);
  v = ((v & 0xff00ff00) >> 8) | ((v & 0x00ff00ff) << 8);
  v = (v >> 16) | (v << 16);
  return v;
}

int main(void){
  cin >> n >> s;
  REP(i, 0, n) {
    cin >> p[i];
  }
  int fw = n / 2, sw = n - n / 2;
  int fw2 = 1 << fw, sw2 = 1 << sw;
  REP(i, 0, fw2) {
    int sum = 0;
    REP(j, 0, fw) {
      if (i & (1 << j)) {
        sum += p[j];
      }
    }
    fi[i] = PI(sum, i);
  }
  REP(i, 0, sw2) {
    int sum = 0;
    REP(j, 0, sw) {
      if (i & (1 << j)) {
        sum += p[fw + j];
      }
    }
    se[i] = PI(sum, i);
  }
  sort(fi, fi + fw2);
  sort(se, se + sw2);
  vector<int> ans;
  int p = 0, q = sw2 - 1;
  for (; p < fw2; ++p) {
    int rest = s - fi[p].first;
    PI *it1 = lower_bound(se, se + sw2, PI(rest,0), Comp());
    PI *it2 = upper_bound(se, se + sw2, PI(rest,0), Comp());
    for (PI *ptr = it1; ptr < it2; ++ptr) {
      ans.push_back(((flip(fi[p].second) >> (32 - fw)) << sw) | flip(ptr->second) >> (32 - sw));
    }
  }
  if (ans.size() >= 51) {
    cerr << "illegal case" << endl;
    exit(4);
  }
  sort(ans.begin(), ans.end());
  reverse(ans.begin(), ans.end());
  REP(i, 0, ans.size()) {
    vector<int> qq;
    REP(j, 0, n) {
      if (ans[i] & (1 << (n - 1 - j))) {
	qq.push_back(j);
      }
    }
    REP(j, 0, qq.size()) {
      cout << qq[j] + 1 << (j == qq.size() - 1 ? "\n" : " ");
    }
  }
  
}

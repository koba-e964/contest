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
struct cmp {
bool operator()(PI a, PI b) const {
return a.first != b.first ? a.first < b.first : a.second > b.second;
}
};

int main(void){
	int n;
cin >> n;
vector<PI> wh(n);
REP(i, 0, n) {
int w,h;
cin >> w >> h;
wh[i] = PI(w, h);
}
sort(wh.begin(), wh.end(), cmp());
VI dp(n + 1, -1);
VI p(n, -1);

//https://en.wikipedia.org/wiki/Longest_increasing_subsequence
int l = 0;
vector<PI> pool;
REP(i, 0, n + 1) {
  if (i == n || (i > 0 && wh[i - 1].first < wh[i].first)) {
    // bang pool
    REP(j, 0, pool.size()) {
      int ip = pool[j].first;
      int lo = pool[j].second;
      if (dp[lo] == -1 || (dp[lo] >= 0 && wh[ip].second < wh[dp[lo]].second)) {
//        cerr << "bang ip=" << ip << ", lo=" << lo << endl;
        p[ip] = dp[lo - 1];
        dp[lo] = ip;
        l = max(lo, l);
      }
    }
    pool.clear();
    if (i == n) break;
  }

  int lo = 1, hi = l;
  while (lo <= hi) {
    int mid = (lo + hi + 1) / 2;
    int mmid = dp[mid];
    if (wh[mmid].first < wh[i].first && wh[mmid].second < wh[i].second) {
      lo = mid + 1;
    } else {
      hi = mid - 1;
    }
  }
  // to pool
//  cerr << "update " << i << " in dp[" << lo << "]" << endl;
  pool.push_back(PI(i, lo));
}
REP(i, 0, n) {
//cerr<<"i=" <<i << ", dp: " << dp[i] << ", p: " << p[i] << endl;
}
//cerr << "dp[n]" << dp[n] << endl;
cout << l << endl;
}

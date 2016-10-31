#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
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
  int n, m, k;
  cin >> n >> m >> k;
  VI x(m), y(m), seat(n);
  REP(i, 0, m) {
    cin >> x[i] >> y[i];
  }
  srand(0xcafebabe);
  int cnt = 0;
  int sim_cnt = 1000000;
  REP(loop_cnt, 0, sim_cnt) {
    REP(i, 0, n) { seat[i] = i; }
    REP(i, 0, k) {
      int f = rand() % n;
      int g = rand() % (n - 1) + 1;
      g = (f + g) % n;
      swap(seat[f], seat[g]);
    }
    int ok = 1;
    REP(i, 0, m) {
      int diff = (seat[x[i]] - seat[y[i]] + n) % n;
      if (diff == 1 || diff == n - 1) {
	ok = 0;
      }
    }
    cnt += ok;
  }
  printf("%.10f\n", double(cnt) / double(sim_cnt));
}

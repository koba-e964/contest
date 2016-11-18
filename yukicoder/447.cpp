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

const int N = 30;
int l[N];

int n;

map<string, int> memo;
vector<string> inv;
vector<VI> score;
vector<int> lastmod;

int get(const string &s) {
  if (memo.count(s)) {
    return memo[s];
  }
  int idx = inv.size();
  memo[s] = idx;
  inv.push_back(s);
  score.push_back(VI(n, 0));
  lastmod.push_back(0);
  return idx;
}

int main(void){
  cin >> n;
  REP(i, 0, n) {
    cin >> l[i];
  }
  int t;
  cin >> t;
  VI already(n, 0);

  REP(i, 0, t) {
    string name, p;
    cin >> name >> p;
    int v = get(name);
    int id = p[0] - 'A';
    int diff = l[id];
    already[id]++;
    int sc = 50 * diff + (500 * diff) / (8 + 2 * already[id]);
    score[v][id] = sc;
    lastmod[v] = i;
  }
  vector<pair<PI, int> > pool;
  REP(i, 0, inv.size()) {
    int sum = 0;
    REP(j, 0, n) {
      sum += score[i][j];
    }
    pool.push_back(make_pair(PI(sum, -lastmod[i]), i));
  }
  sort(pool.rbegin(), pool.rend());
  REP(i, 0, inv.size()) {
    int idx = pool[i].second;
    cout << i + 1 << " " << inv[idx] << " ";
    REP(j, 0, n) {
      cout << score[idx][j] << " ";
    }
    cout << pool[i].first.first << endl;
  }
}

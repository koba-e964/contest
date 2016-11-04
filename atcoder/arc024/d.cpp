#include <algorithm>
#include <bitset>
#include <cassert>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
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

const int N = 1000;
int bitmap[N][N];
VI cols[N];

// [l, r], [start, end)
vector<PI> solve(int l, int r) {
  if (l > r) {
    return vector<PI>();
  }
  int mid = (l + r) / 2;
  set<int> ys;
  REP(i, l, r + 1) {
    REP(j, 0, cols[i].size()) {
      if (not bitmap[mid][cols[i][j]]) {
	ys.insert(cols[i][j]);
      }
    }
  }
  vector<PI> colpt;
  for (set<int>::iterator it = ys.begin(); it != ys.end(); ++it) {
    colpt.push_back(PI(mid, *it));
  }
  vector<PI> left = solve(l, mid - 1);
  vector<PI> right = solve(mid + 1, r);
  if (left.size() < right.size()) {
    swap(left, right);
  }
  REP(i, 0, right.size()) {
    left.push_back(right[i]);
  }
  REP(i, 0, colpt.size()) {
    left.push_back(colpt[i]);
  }
  return left;
}

int main(void){
  int n;
  cin >> n;
  VI x(n), y(n);
  REP(i, 0, n) {
    cin >> x[i] >> y[i];
    x[i]--, y[i]--;
    bitmap[x[i]][y[i]] = 1;
    cols[x[i]].push_back(y[i]);
  }
  vector<PI> addition = solve(0, N - 1);
  cout << addition.size() << endl;
  REP(i, 0, addition.size()) {
    PI d = addition[i];
    cout << d.first + 1 << " " << d.second + 1 << endl;
  }
}

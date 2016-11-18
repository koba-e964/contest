#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;


/**
 * Binary Indexed Tree (Fenwick Tree). Holds an array of type T.
 * T is a commutative monoid. Indices are 1 .. n.
 * Verified by AtCoder ARC043 C (http://arc043.contest.atcoder.jp/submissions/985157)
 */
template <class T, class Op> class BIT {
private:
  int n;
  std::vector<T> ary;
  Op op;
  T e;
public:
  BIT(int n, Op op = Op(), T e = T()) : op(op), e(e) {
    assert (n > 0);
    while(n != (n & (-n))) { // find the least power of 2 >= n
      n += n & (-n);
    }
    this->n = n;
    ary = std::vector<T>(n + 1);
    for(int i = 0; i <= n; i++) {
      ary[i] = e;
    }
  }
  /**
   * gets the sum in [1 .. idx]
   * @param idx
   * @return sum
   */
  T accum(int idx) {
    T sum = e;
    while(idx > 0) {
      sum = op(sum, ary[idx]);
      idx &= idx - 1;
    }
    return sum;
  }
  /**
   * performs data[idx] += val;
   */
  void add(int idx, T val) {
    assert (idx > 0);
    while(idx <= n) {
      ary[idx] = op(ary[idx], val);
      idx += idx & (-idx);
    }
  }
};

int l[30];

int n;

map<string, int> memo;
vector<string> inv;
vector<VI> score;
vector<int> lastmod;
vector<int> sum;

int get(const string &s) {
  if (memo.count(s)) {
    return memo[s];
  }
  int idx = inv.size();
  memo[s] = idx;
  inv.push_back(s);
  score.push_back(VI(n, 0));
  lastmod.push_back(0);
  sum.push_back(0);
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
  typedef pair<ll, ll> PL;
  vector<PL> events;
  VL sorter;

  REP(i, 0, t) {
    string name, p;
    cin >> name >> p;
    int v = get(name);
    if (p == "?") {
      events.push_back(PL(v, -1));
      continue;
    }
    int id = p[0] - 'A';
    int diff = l[id];
    already[id]++;
    int sc = 50 * diff + (500 * diff) / (8 + 2 * already[id]);
    score[v][id] = sc;
    lastmod[v] = i;
    sum[v] += sc;
    events.push_back(PL(v, (ll)sum[v] * 1e9 - i));
    sorter.push_back((ll)sum[v] * 1e9 - i);
  }
  sort(sorter.begin(), sorter.end());
  map<ll, int> sinv;
  REP(i, 0, sorter.size()) {
    sinv[sorter[i]] = i;
  }
  REP(i, 0, events.size()) {
    if (events[i].second >= 0) {
      events[i].second = sinv[events[i].second];
    }
  }
  BIT<int, plus<int> > bit(2 * t + 1);
  VL cur(inv.size(), -1);
  REP(i, 0, events.size()) {
    int v = events[i].first;
    int sc = events[i].second;
    if (0) {
      cerr << "query: " << v << " " << sc << endl;
    }
    if (sc >= 0) {
      if (cur[v] >= 0) {
	bit.add(cur[v] + 1, -1);
      }
      bit.add(sc + 1, 1);
      cur[v] = sc;
    } else {
      cout << 1 + bit.accum(2 * t) - bit.accum(cur[v] + 1) << endl;
    }
  }
}

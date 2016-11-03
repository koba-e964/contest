#include <algorithm>
#include <cassert>
#include <cmath>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int lim = 1e9;


const int PT = 100000;
int prime[PT];

const int Q = 9;

struct state {
  VI del;
  int val;
  int nf;
  ll score;
  bool operator < (const state &o) const {
    return score < o.score;
  }
};



PI beam_search(int q) {
  int fact[Q] = {2,3,5,7,11,13,17,19,23};
  VI init_del = VI(Q, 1);
  vector<state> que;
  vector<state> next;
  state mast = {VI(Q, 0), 0, 0, 0};
  que.push_back((state) {init_del, 1, 1, 1});
  int gen = 0;
  while (not que.empty()) {
    //cerr << "generation: " << gen << endl;
    gen++;
    REP(i, 0, que.size()) {
      next.clear();
      state st = que[i];
      //cerr << "q = " << q << ", st.val=" << st.val << ", st.nf=" << st.nf << endl;
      if (st.nf > mast.nf) {
	mast = st;
      }
      double ls = 0;
      REP(i, 0, Q) {
	ls += exp(-pow(st.del[i] - 1, 2));
      }
      REP(i, 0, Q) {
	int tmp = 100000 / log(fact[i]) * log(1 + 1.0 / st.del[i]);
	if (q >= (ll) st.val * fact[i]) {
	  state nexst = {st.del, st.val, st.nf, st.score};
	  nexst.val *= fact[i];
	  nexst.nf *= 1 + st.del[i];
	  nexst.nf /= st.del[i];
	  nexst.del[i]++;
	  nexst.score += ls * tmp;
	  next.push_back(nexst);
	}
      }
    }
    sort(next.rbegin(), next.rend());
    // retain 10 best ones
    que = vector<state> (next.begin(), next.begin() + min(10, (int)next.size()));
  }
  return PI(mast.nf, mast.val);
}

PI monetize(int v) {
  VI del(Q, 1);
  int q = lim / v;
  PI result = beam_search(q);
  return PI(result.first, v * result.second);
}

int main(void){
  REP(i, 2, PT) {
    prime[i] = 1;
  }

  REP(i, 2, sqrt(PT) + 1) {
    if (not prime[i]) { continue; }
    REP(j, 2, PT / i) {
      prime[i * j] = 0;
    }
  }
  VI med_pr;
  REP(i, 29, 1000) {
    if (prime[i]) med_pr.push_back(i);
  }
  sort(med_pr.begin(), med_pr.end());
  assert (med_pr.size() >= 100);
  vector<PI> result(med_pr.size());
  REP(i, 0, med_pr.size()) {
    result[i] = monetize(med_pr[i]);
  }
  sort(result.rbegin(), result.rend());
  REP(i, 0, 100) {
    //cout << "seed=" << med_pr[i] << endl;
    cout << result[i].second <<
      " " << result[i].first <<
      endl;
  }
}

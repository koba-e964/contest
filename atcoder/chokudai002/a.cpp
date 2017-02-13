#include <algorithm>
#include <set>
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

const int lim = 1e9;


void output(const vector<int> &result) {
  assert (result.size() == 100);
  REP(i, 0, 100) {
    assert (result[i] >= 1);
    assert (result[i] <= lim);
  }
#ifndef DEBUG
  REP(i, 0, 100) {
    cout << result[i] << endl;
  }
#else
  {
    set<int> factors;
    REP(i, 0, 100) {
      int tmp = factors.size();
      int v = result[i];
      for (int k = 1; k * k <= v; ++k) {
	if (v % k == 0) {
	  factors.insert(k);
	  factors.insert(v / k);
	}
      }
    }
    cout << "score = " << factors.size() << endl;
  }
#endif
} 
 
const int PT = 100000;
int prime[PT];
 
int monetize(int v) {
  const int Q = 9;
  int fact[Q] = {2,3,5,7,11,13,17,19,23};
  int del[Q] = {1,1,1,1,1,1,1,1,1};
  ll k = v;
  //ll k = v * 32 * 9 * 5 * 7 * 11 * 13;
  while (1) {
    int q = lim / k;
    int ma = 0, maxi = -1;
    REP(i, 0, Q) {
      int tmp = 100000 / log(fact[i]) / del[i];
      if (q >= fact[i] && ma < tmp) {
	ma = tmp;
	maxi = i;
      }
    }
    if (maxi == -1) break;
    k *= fact[maxi];
    del[maxi]++;
  }
  return k;
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
  VI small_pr;
  REP(p, 7, 38) {
    if (prime[p]) small_pr.push_back(p);
  }
  int spr_size = small_pr.size();
  VI med_pr;
  REP(i, small_pr[spr_size - 1] + 1, 1000) {
    if (prime[i]) med_pr.push_back(i);
  }
  REP(i, 13, 1000) {
    REP(j, 23, 1000) {
      if (prime[i] && prime[j]) {
	med_pr.push_back(i * j);
      }
    }
  }
  sort(med_pr.begin(), med_pr.end());
  assert (med_pr.size() >= 100 - spr_size);
  VI result;
  REP(i, 0, spr_size) {
    result.push_back(monetize(64 * 27 * 25 * 7 * 11 * small_pr[i]));
  }
  REP(i, 0, 100 - spr_size) {
    result.push_back(monetize(med_pr[i]));
  }
#ifdef DEBUG
  cout << "small_pr.size() = " << small_pr.size() << endl;
  cout << "med_pr.size() = " << med_pr.size() << endl;
#endif
  output(result);
}

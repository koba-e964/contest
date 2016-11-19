#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <string>
#include <vector>
#include <bitset>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

const int N=150;

int e[170][170];

int main(void){
  int a = 13;
  int b = 13;
  REP(i, 0, b) {
    REP(k, 0, a) {
      e[i][a * i + k] = 1;
    }
  }
  REP(step, 0, a) {
    REP(init, 0, a) {
      REP(i, 0, b) {
	if (step + a * init + b < N) {
	  e[step + a * init + b][i * b + ((step * i + init) % a)] = 1;
	}
      }
    }
  }
  cout << N << endl;
  REP(i, 0, N) {
    REP(j, 0, N) {
      cout << (e[i][j] ? 'O' : '.');
    }
    cout << endl;
  }
}

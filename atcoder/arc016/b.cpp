#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const int N=100;
string board[N];


int main(void){
	int n;
	cin>>n;
	REP(i,0,n){
		cin>>board[i];
	}
	int c=0;
	REP(r,0,9){
		REP(i,0,n){
			switch(board[i][r]){
			case '.':
				break;
			case 'x':
				c++;
				break;
			case 'o':
				if(i==n-1 ||board[i+1][r]!='o'){
					c++;
				}
				break;
			default:
				assert(0);
			}
		}
	}
	cout<<c<<endl;
}

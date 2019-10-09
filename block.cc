#include "block.h"

Block::Block()
{

}

Block::Block(int nBatch, int nStage, int nType, int nDir)
{
    this->nBatch = nBatch;
    this->nStage = nStage;
    this->nType = nType;
    this->nDir = nDir;
}

int Block::getBatch() { return this->nBatch; }
int Block::getStage() { return this->nStage; }
int Block::getType() { return this->nType; }
int Block::getDir() { return this->nDir; }

void Block::setDuration(Duration d) {this->dur = d;}
Duration Block::getDuration() {return this->dur;}

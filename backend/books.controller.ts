import { Controller, Post, Body } from '@nestjs/common';
import { BooksService } from './books.service';

@Controller('books')
export class BooksController {
  constructor(private readonly booksService: BooksService) {}

  @Post('initialize')
  async initialize(@Body() body) {
    const { rentPricePerDay } = body;
    const tx = await this.booksService.initializeBookRental(rentPricePerDay);
    return { tx };
  }

  @Post('rent')
  async rentBook(@Body() body) {
    const { renter, days } = body;
    const tx = await this.booksService.rentBook(renter, days);
    return { tx };
  }

  @Post('return')
  async returnBook(@Body() body) {
    const { renter } = body;
    const tx = await this.booksService.returnBook(renter);
    return { tx };
  }
}
